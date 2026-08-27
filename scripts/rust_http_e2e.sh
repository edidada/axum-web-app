#!/usr/bin/env bash

set -euo pipefail

# The test is intentionally self-contained: it owns a temporary SQLite database,
# a unique Redis key prefix, and the Rust server process that it starts.
unset HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy
export NO_PROXY="127.0.0.1,localhost"
export no_proxy="$NO_PROXY"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
E2E_PORT="${E2E_PORT:-19002}"
BASE_URL="http://127.0.0.1:${E2E_PORT}"
REDIS_HOST="${E2E_REDIS_HOST:-127.0.0.1}"
REDIS_PORT="${E2E_REDIS_PORT:-6379}"
REDIS_PREFIX="axum:e2e:${$}:"
RUN_DIR="$(mktemp -d "${TMPDIR:-/tmp}/axum-rust-e2e.XXXXXX")"
DATABASE_FILE="$RUN_DIR/e2e.db"
COOKIE_JAR="$RUN_DIR/cookies.txt"
SERVER_LOG="$RUN_DIR/server.log"
RESPONSE_FILE="$RUN_DIR/response.json"
SERVER_PID=""
CSRF_TOKEN=""
REQUEST_COUNT=0

cleanup_redis_keys() {
    redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" --raw --scan \
        --pattern "${REDIS_PREFIX}*" 2>/dev/null | while IFS= read -r key; do
        if [ -n "$key" ]; then
            redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" DEL "$key" >/dev/null
        fi
    done
}

cleanup() {
    local exit_code=$?
    if [ -n "$SERVER_PID" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        kill "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    cleanup_redis_keys || true
    if [ "$exit_code" -ne 0 ]; then
        echo "E2E failed; Rust server log follows:" >&2
        tail -n 120 "$SERVER_LOG" >&2 || true
    fi
    rm -rf -- "$RUN_DIR"
    exit "$exit_code"
}
trap cleanup EXIT INT TERM

fail() {
    echo "ERROR: $*" >&2
    exit 1
}

assert_contains() {
    local file=$1
    local expected=$2
    grep -Fq -- "$expected" "$file" || fail "response does not contain '$expected': $(cat "$file")"
}

assert_not_contains() {
    local file=$1
    local unexpected=$2
    if grep -Fq -- "$unexpected" "$file"; then
        fail "response still contains deleted value '$unexpected': $(cat "$file")"
    fi
}

perform_request() {
    local label=$1
    local method=$2
    local path=$3
    local expected_status=$4
    local body=${5:-}
    local content_type=${6:-application/json}
    local auth_mode=${7:-auth}
    local status
    local -a curl_args

    curl_args=(
        curl --silent --show-error --noproxy "*"
        --connect-timeout 3 --max-time 20
        --output "$RESPONSE_FILE" --write-out "%{http_code}"
        --request "$method" "$BASE_URL$path"
    )
    if [ "$auth_mode" = "auth" ]; then
        curl_args+=(
            --cookie "$COOKIE_JAR"
            --header "Origin: http://127.0.0.1:5173"
            --header "x-csrf-token: $CSRF_TOKEN"
        )
    fi
    if [ -n "$body" ]; then
        curl_args+=(--header "Content-Type: $content_type" --data "$body")
    fi

    status="$("${curl_args[@]}")"
    REQUEST_COUNT=$((REQUEST_COUNT + 1))
    if [ "$status" != "$expected_status" ]; then
        fail "$label expected HTTP $expected_status, got $status: $(cat "$RESPONSE_FILE")"
    fi
    printf '[%03d] %-44s HTTP %s\n' "$REQUEST_COUNT" "$label" "$status"
}

extract_inserted_id() {
    sed -n 's/.*"id":"\([^"]*\)".*/\1/p' "$RESPONSE_FILE" | head -n 1
}

crud_cycle() {
    local label=$1
    local route=$2
    local select_body=$3
    local insert_body=$4
    local update_template=$5
    local before_marker=$6
    local after_marker=$7
    local record_id
    local update_body

    perform_request "$label 1/6 insert" POST "/$route/insert" 200 "$insert_body"
    record_id="$(extract_inserted_id)"
    [ -n "$record_id" ] || fail "$label insert response did not contain an id"

    perform_request "$label 2/6 select after insert" POST "/$route/select-page" 200 "$select_body"
    assert_contains "$RESPONSE_FILE" "$record_id"
    assert_contains "$RESPONSE_FILE" "$before_marker"

    update_body="${update_template/__ID__/$record_id}"
    perform_request "$label 3/6 update" POST "/$route/update" 200 "$update_body"
    assert_contains "$RESPONSE_FILE" "$after_marker"

    perform_request "$label 4/6 select after update" POST "/$route/select-page" 200 "$select_body"
    assert_contains "$RESPONSE_FILE" "$record_id"
    assert_contains "$RESPONSE_FILE" "$after_marker"

    perform_request "$label 5/6 delete" POST "/$route/delete" 200 "\"$record_id\""
    assert_contains "$RESPONSE_FILE" '"success":true'

    perform_request "$label 6/6 select after delete" POST "/$route/select-page" 200 "$select_body"
    assert_not_contains "$RESPONSE_FILE" "$record_id"
}

command -v curl >/dev/null || fail "curl is required"
command -v sqlite3 >/dev/null || fail "sqlite3 is required"
command -v redis-cli >/dev/null || fail "redis-cli is required"
command -v cargo >/dev/null || fail "cargo is required"

redis_ping="$(redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" ping 2>/dev/null || true)"
[ "$redis_ping" = "PONG" ] || fail "Redis is not ready at $REDIS_HOST:$REDIS_PORT"
cleanup_redis_keys

sqlite3 "$DATABASE_FILE" <<'SQL'
CREATE TABLE sys_dict (
    id TEXT PRIMARY KEY, name TEXT, code TEXT, value TEXT, sort INTEGER,
    remark TEXT, state INTEGER, create_date TEXT
);
CREATE TABLE address_book (
    id TEXT PRIMARY KEY, user_id TEXT, consignee TEXT, sex INTEGER, phone TEXT,
    province_code TEXT, province_name TEXT, city_code TEXT, city_name TEXT,
    district_code TEXT, district_name TEXT, detail TEXT, label TEXT,
    is_default INTEGER, create_time TEXT, update_time TEXT,
    create_user TEXT, update_user TEXT, is_deleted INTEGER
);
CREATE TABLE category (
    id TEXT PRIMARY KEY, category_type INTEGER, name TEXT, sort INTEGER,
    create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT
);
CREATE TABLE dish (
    id TEXT PRIMARY KEY, name TEXT, category_id TEXT, price TEXT, code TEXT,
    image TEXT, description TEXT, status INTEGER, sort INTEGER,
    create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT,
    is_deleted INTEGER
);
CREATE TABLE dish_flavor (
    id TEXT PRIMARY KEY, dish_id TEXT, name TEXT, value TEXT,
    create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT,
    is_deleted INTEGER
);
CREATE TABLE employee (
    id TEXT PRIMARY KEY, name TEXT, username TEXT, password TEXT, phone TEXT,
    sex INTEGER, id_number TEXT, status INTEGER, create_time TEXT,
    update_time TEXT, create_user TEXT, update_user TEXT
);
CREATE TABLE order_detail (
    id TEXT PRIMARY KEY, name TEXT, image TEXT, order_id TEXT, dish_id TEXT,
    setmeal_id TEXT, dish_flavor TEXT, number INTEGER, amount TEXT
);
CREATE TABLE orders (
    id TEXT PRIMARY KEY, number TEXT, status INTEGER, user_id TEXT,
    address_book_id TEXT, order_time TEXT, checkout_time TEXT,
    pay_method INTEGER, pay_status INTEGER, amount TEXT, remark TEXT,
    phone TEXT, address TEXT, user_name TEXT, consignee TEXT,
    cancel_reason TEXT, rejection_reason TEXT, cancel_time TEXT,
    estimated_delivery_time TEXT, delivery_status INTEGER, delivery_time TEXT,
    pack_amount INTEGER, tableware_number INTEGER, tableware_status INTEGER
);
CREATE TABLE setmeal (
    id TEXT PRIMARY KEY, category_id TEXT, name TEXT, price TEXT,
    status INTEGER, code TEXT, description TEXT, image TEXT,
    create_time TEXT, update_time TEXT, create_user TEXT, update_user TEXT,
    is_deleted INTEGER
);
CREATE TABLE setmeal_dish (
    id TEXT PRIMARY KEY, setmeal_id TEXT, dish_id TEXT, name TEXT, price TEXT,
    copies INTEGER, sort INTEGER, create_time TEXT, update_time TEXT,
    create_user TEXT, update_user TEXT
);
CREATE TABLE shopping_cart (
    id TEXT PRIMARY KEY, name TEXT, image TEXT, user_id TEXT, dish_id TEXT,
    setmeal_id TEXT, dish_flavor TEXT, number INTEGER, amount TEXT,
    create_time TEXT
);
CREATE TABLE "user" (
    id TEXT PRIMARY KEY, openid TEXT, name TEXT, phone TEXT, sex INTEGER,
    id_number TEXT, avatar TEXT, status INTEGER, is_deleted INTEGER
);

INSERT INTO sys_dict (id,name,code,value,sort,remark,state,create_date)
VALUES ('seed-dict','Seed dictionary','seed-code','seed-value',1,'report fixture',1,'2026-01-01 00:00:00');
INSERT INTO "user" (id,openid,name,phone,sex,id_number,avatar,status,is_deleted)
VALUES ('seed-user','seed-openid','Seed user','13800000000',1,'seed-id','',1,0);
INSERT INTO address_book (id,user_id,consignee,sex,phone,detail,label,is_default,create_time,is_deleted)
VALUES ('seed-address','seed-user','Seed consignee',1,'13800000000','Seed road','home',1,'2026-01-01 00:00:00',0);
INSERT INTO category (id,category_type,name,sort,create_time)
VALUES ('seed-category',1,'Seed category',1,'2026-01-01 00:00:00');
INSERT INTO dish (id,name,category_id,price,code,status,sort,create_time,is_deleted)
VALUES ('seed-dish','Seed dish','seed-category','12.50','seed-dish-code',1,1,'2026-01-01 00:00:00',0);
INSERT INTO dish_flavor (id,dish_id,name,value,create_time,is_deleted)
VALUES ('seed-flavor','seed-dish','spice','mild','2026-01-01 00:00:00',0);
INSERT INTO employee (id,name,username,password,phone,status,create_time)
VALUES ('seed-employee','Seed employee','seed-employee','secret','13900000000',1,'2026-01-01 00:00:00');
INSERT INTO orders (id,number,status,user_id,address_book_id,order_time,pay_status,amount,consignee)
VALUES ('seed-order','SEED-ORDER',1,'seed-user','seed-address','2026-01-01 00:00:00',1,'12.50','Seed consignee');
INSERT INTO order_detail (id,name,order_id,dish_id,number,amount)
VALUES ('seed-detail','Seed dish','seed-order','seed-dish',1,'12.50');
INSERT INTO setmeal (id,category_id,name,price,status,code,create_time,is_deleted)
VALUES ('seed-setmeal','seed-category','Seed setmeal','20.00',1,'seed-setmeal-code','2026-01-01 00:00:00',0);
INSERT INTO setmeal_dish (id,setmeal_id,dish_id,name,price,copies,sort,create_time)
VALUES ('seed-setmeal-dish','seed-setmeal','seed-dish','Seed dish','12.50',1,1,'2026-01-01 00:00:00');
INSERT INTO shopping_cart (id,name,user_id,dish_id,number,amount,create_time)
VALUES ('seed-cart','Seed dish','seed-user','seed-dish',1,'12.50','2026-01-01 00:00:00');
SQL

cd "$PROJECT_DIR"
(
    export APP_PROFILE=sqlite
    export APP_WEBAPP__PORT="$E2E_PORT"
    export APP_DATABASE__URL="sqlite://$DATABASE_FILE"
    export APP_SESSION__BACKEND=redis
    export APP_SESSION__REDIS_URL="redis://$REDIS_HOST:$REDIS_PORT/0"
    export APP_SESSION__REDIS_KEY_PREFIX="$REDIS_PREFIX"
    cargo run --offline --bin main
) >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!

server_ready=false
attempt=1
while [ "$attempt" -le 120 ]; do
    if curl --silent --show-error --noproxy "*" --max-time 2 \
        "$BASE_URL/health" >/dev/null 2>&1; then
        server_ready=true
        break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
        fail "Rust server exited before becoming ready"
    fi
    sleep 1
    attempt=$((attempt + 1))
done
[ "$server_ready" = true ] || fail "Rust server did not become ready at $BASE_URL"

perform_request "unauthenticated protected route" GET /get 401 "" application/json noauth
perform_request "health" GET /health 200 "" application/json noauth
assert_contains "$RESPONSE_FILE" '"status":"healthy"'
perform_request "detailed health" GET /health/detailed 200 "" application/json noauth
assert_contains "$RESPONSE_FILE" '"database_connected":true'
perform_request "readiness" GET /ready 200 "" application/json noauth
assert_contains "$RESPONSE_FILE" '"ready":true'
perform_request "liveness" GET /live 200 "" application/json noauth
assert_contains "$RESPONSE_FILE" '"alive":true'

perform_request "invalid login" POST /api/login 401 \
    '{"username":"wrong","password":"wrong"}' application/json noauth

login_status="$(curl --silent --show-error --noproxy "*" \
    --connect-timeout 3 --max-time 20 \
    --cookie-jar "$COOKIE_JAR" --output "$RESPONSE_FILE" --write-out "%{http_code}" \
    --request POST "$BASE_URL/api/login" \
    --header 'Origin: http://127.0.0.1:5173' \
    --header 'Content-Type: application/json' \
    --data '{"username":"edidada","password":"Pass123Qwe"}')"
REQUEST_COUNT=$((REQUEST_COUNT + 1))
[ "$login_status" = 200 ] || fail "valid login expected HTTP 200, got $login_status: $(cat "$RESPONSE_FILE")"
printf '[%03d] %-44s HTTP %s\n' "$REQUEST_COUNT" "valid login using Redis session" "$login_status"
assert_contains "$RESPONSE_FILE" '"role":"admin"'

CSRF_TOKEN="$(awk '$6 == "csrf_token" { print $7 }' "$COOKIE_JAR" | tail -n 1)"
[ -n "$CSRF_TOKEN" ] || fail "login did not create the CSRF cookie"
redis_session_count="$(redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" --raw --scan \
    --pattern "${REDIS_PREFIX}*" | wc -l | tr -d ' ')"
[ "$redis_session_count" -ge 1 ] || fail "login did not create a Redis session key"

perform_request "current user" GET /api/me 200
assert_contains "$RESPONSE_FILE" '"username":"edidada"'
perform_request "legacy GET and XSS sanitization" GET /get 200
assert_contains "$RESPONSE_FILE" 'Hello, World!'
assert_not_contains "$RESPONSE_FILE" '<script>'
perform_request "legacy form POST" POST /post 200 \
    'custom_username=AuditUser&custom_password=not-logged' application/x-www-form-urlencoded
assert_contains "$RESPONSE_FILE" 'AuditUser'
assert_not_contains "$RESPONSE_FILE" 'not-logged'
perform_request "legacy JSON body sanitization" POST /getBody 200 \
    '{"html_code":"<h1>safe-e2e</h1><script>alert(1)</script>"}'
assert_contains "$RESPONSE_FILE" 'safe-e2e'
assert_not_contains "$RESPONSE_FILE" '<script>'
perform_request "raw pagination" POST /paginate 200 \
    '{"sql":"SELECT id,name FROM sys_dict ORDER BY id","page":1,"page_size":10}'
assert_contains "$RESPONSE_FILE" 'seed-dict'
perform_request "parameterized pagination" POST /paginate-with-params 200 \
    '{"sql":"SELECT id,name FROM sys_dict WHERE code = ?","page":1,"page_size":10,"params":["seed-code"]}'
assert_contains "$RESPONSE_FILE" 'seed-dict'

crud_cycle "sys-dict" "sys-dict" \
    '{"page":1,"size":100,"code":null,"name":null}' \
    '{"name":"crud-dict-before","code":"crud-dict-code","value":"one","sort":2,"remark":"e2e","state":1}' \
    '{"id":"__ID__","name":"crud-dict-after","code":"crud-dict-code","value":"two","sort":3,"remark":"e2e-updated","state":1}' \
    'crud-dict-before' 'crud-dict-after'
perform_request "sys-dict compatibility list" POST /api/sys-dict/list 200 '{}'
assert_contains "$RESPONSE_FILE" 'seed-dict'

PAGE_BODY='{"page":1,"page_size":100}'
crud_cycle "address-book" "address-book" "$PAGE_BODY" \
    '{"user_id":"crud-user","consignee":"address-before","phone":"13811110000","detail":"Before road","label":"home","is_default":0,"is_deleted":0}' \
    '{"id":"__ID__","user_id":"crud-user","consignee":"address-after","phone":"13811110000","detail":"After road","label":"office","is_default":1,"is_deleted":0}' \
    'address-before' 'address-after'
crud_cycle "category" "category" "$PAGE_BODY" \
    '{"category_type":1,"name":"category-before","sort":10}' \
    '{"id":"__ID__","category_type":2,"name":"category-after","sort":20}' \
    'category-before' 'category-after'
crud_cycle "dish" "dish" "$PAGE_BODY" \
    '{"name":"dish-before","category_id":"seed-category","price":"18.00","code":"crud-dish","status":1,"sort":10,"is_deleted":0}' \
    '{"id":"__ID__","name":"dish-after","category_id":"seed-category","price":"19.00","code":"crud-dish","status":1,"sort":20,"is_deleted":0}' \
    'dish-before' 'dish-after'
crud_cycle "dish-flavor" "dish-flavor" "$PAGE_BODY" \
    '{"dish_id":"seed-dish","name":"flavor-before","value":"mild","is_deleted":0}' \
    '{"id":"__ID__","dish_id":"seed-dish","name":"flavor-after","value":"hot","is_deleted":0}' \
    'flavor-before' 'flavor-after'
crud_cycle "employee" "employee" "$PAGE_BODY" \
    '{"name":"employee-before","username":"crud-employee","password":"secret","phone":"13911110000","status":1}' \
    '{"id":"__ID__","name":"employee-after","username":"crud-employee","password":"secret","phone":"13911110000","status":1}' \
    'employee-before' 'employee-after'
crud_cycle "order-detail" "order-detail" "$PAGE_BODY" \
    '{"name":"detail-before","order_id":"seed-order","dish_id":"seed-dish","number":1,"amount":"12.50"}' \
    '{"id":"__ID__","name":"detail-after","order_id":"seed-order","dish_id":"seed-dish","number":2,"amount":"25.00"}' \
    'detail-before' 'detail-after'
crud_cycle "orders" "orders" "$PAGE_BODY" \
    '{"number":"order-before","status":1,"user_id":"seed-user","amount":"20.00","consignee":"E2E"}' \
    '{"id":"__ID__","number":"order-after","status":2,"user_id":"seed-user","amount":"21.00","consignee":"E2E"}' \
    'order-before' 'order-after'
crud_cycle "setmeal" "setmeal" "$PAGE_BODY" \
    '{"category_id":"seed-category","name":"setmeal-before","price":"30.00","status":1,"code":"crud-setmeal","is_deleted":0}' \
    '{"id":"__ID__","category_id":"seed-category","name":"setmeal-after","price":"31.00","status":1,"code":"crud-setmeal","is_deleted":0}' \
    'setmeal-before' 'setmeal-after'
crud_cycle "setmeal-dish" "setmeal-dish" "$PAGE_BODY" \
    '{"setmeal_id":"seed-setmeal","dish_id":"seed-dish","name":"setmeal-dish-before","price":"12.50","copies":1}' \
    '{"id":"__ID__","setmeal_id":"seed-setmeal","dish_id":"seed-dish","name":"setmeal-dish-after","price":"12.50","copies":2}' \
    'setmeal-dish-before' 'setmeal-dish-after'
crud_cycle "shopping-cart" "shopping-cart" "$PAGE_BODY" \
    '{"name":"cart-before","user_id":"seed-user","dish_id":"seed-dish","number":1,"amount":"12.50"}' \
    '{"id":"__ID__","name":"cart-after","user_id":"seed-user","dish_id":"seed-dish","number":2,"amount":"25.00"}' \
    'cart-before' 'cart-after'
crud_cycle "user" "user" "$PAGE_BODY" \
    '{"name":"user-before","phone":"13711110000","sex":1,"id_number":"crud-user-id","status":1}' \
    '{"id":"__ID__","name":"user-after","phone":"13711110000","sex":1,"id_number":"crud-user-id","status":1}' \
    'user-before' 'user-after'

perform_request "report order details" GET '/report/order-details?order_id=seed-order' 200
assert_contains "$RESPONSE_FILE" 'seed-detail'
perform_request "report setmeal dishes" GET '/report/setmeal-dishes?setmeal_id=seed-setmeal' 200
assert_contains "$RESPONSE_FILE" 'seed-setmeal'
perform_request "report shopping cart" GET '/report/shopping-cart?cart_id=seed-cart' 200
assert_contains "$RESPONSE_FILE" 'seed-cart'
perform_request "report dish full info" GET '/report/dish-full-info?dish_id=seed-dish' 200
assert_contains "$RESPONSE_FILE" 'seed-dish'
perform_request "report user order stats" GET '/report/user-order-stats?user_id=seed-user' 200
assert_contains "$RESPONSE_FILE" 'seed-user'
perform_request "report address book" GET '/report/address-book?address_id=seed-address' 200
assert_contains "$RESPONSE_FILE" 'seed-address'

perform_request "logout and revoke Redis session" POST /api/logout 200
assert_contains "$RESPONSE_FILE" '"status":"success"'
perform_request "replay after logout" GET /api/me 401

remaining_keys="$(redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" --raw --scan \
    --pattern "${REDIS_PREFIX}*" | wc -l | tr -d ' ')"
[ "$remaining_keys" -eq 0 ] || fail "logout left $remaining_keys Redis session key(s)"

echo "PASS: $REQUEST_COUNT HTTP checks covered every default Rust route with Redis sessions."
