# ============================================================
# axum-web-app 全接口测试脚本
# 用法: powershell -ExecutionPolicy Bypass -File test_all_api.ps1 [-BaseUrl http://127.0.0.1:9002]
# 覆盖: 健康检查 / 登录 / 旧测试接口 / 12 个实体 CRUD / 报表 / 登出
# ============================================================
param(
    [string]$BaseUrl = "http://127.0.0.1:9002",
    [string]$Username = "edidada",
    [string]$Password = "Pass123Qwe"
)

$ErrorActionPreference = "Continue"
$results = [System.Collections.Generic.List[object]]::new()
$sessionToken = $null

function Write-Test {
    param([string]$Method, [string]$Path, [int]$Status, [string]$Body, [bool]$Expect2xx = $true)
    $pass = ($Expect2xx -and $Status -ge 200 -and $Status -lt 300) -or (-not $Expect2xx)
    $short = if ($Body.Length -gt 80) { $Body.Substring(0, 80) + "..." } else { $Body }
    $results.Add([pscustomobject]@{ Method = $Method; Path = $Path; Status = $Status; Body = $short; Pass = $pass })
    $mark = if ($pass) { "PASS" } else { "FAIL" }
    Write-Host ("[{0}] {1} {2} -> {3}" -f $mark, $Method, $Path, $Status) -ForegroundColor $(if ($pass) { "Green" } else { "Red" })
    if (-not $pass) { Write-Host ("       " + $short) }
}

function Invoke-Api {
    param(
        [string]$Method = "GET",
        [string]$Path,
        [object]$Body = $null,
        [bool]$Auth = $true,
        [bool]$Expect2xx = $true,
        [string]$ContentType = "application/json"
    )
    $headers = @{}
    if ($Auth -and $sessionToken) { $headers["Authorization"] = "Bearer $sessionToken" }
    $params = @{ Uri = "$BaseUrl$Path"; Method = $Method; Headers = $headers; UseBasicParsing = $true; TimeoutSec = 15 }
    if ($null -ne $Body) {
        if ($ContentType -eq "application/json") {
            $params["ContentType"] = "application/json"
            $params["Body"] = ($Body | ConvertTo-Json -Depth 10 -Compress)
        } else {
            $params["ContentType"] = $ContentType
            $params["Body"] = $Body
        }
    }
    try {
        $resp = Invoke-WebRequest @params
        Write-Test -Method $Method -Path $Path -Status $resp.StatusCode -Body $resp.Content -Expect2xx $Expect2xx
        return $resp.Content
    } catch {
        $code = 0
        if ($_.Exception.Response) { $code = [int]$_.Exception.Response.StatusCode }
        Write-Test -Method $Method -Path $Path -Status $code -Body $_.Exception.Message -Expect2xx $Expect2xx
        return $null
    }
}

Write-Host "==================== 1. 健康检查 ====================" -ForegroundColor Cyan
Invoke-Api -Method GET -Path "/health" -Auth $false
Invoke-Api -Method GET -Path "/health/detailed" -Auth $false
Invoke-Api -Method GET -Path "/ready" -Auth $false
Invoke-Api -Method GET -Path "/live" -Auth $false

Write-Host "==================== 2. 登录 ====================" -ForegroundColor Cyan
$loginBody = Invoke-Api -Method POST -Path "/api/login" -Body @{ username = $Username; password = $Password } -Auth $false
if ($loginBody) {
    try { $loginJson = $loginBody | ConvertFrom-Json } catch { $loginJson = $null }
    if ($loginJson -and $loginJson.session_id) {
        $sessionToken = $loginJson.session_id
        Write-Host "       登录成功, session_id: $($sessionToken.Substring(0,12))..." -ForegroundColor Green
    } else {
        Write-Host "       登录响应缺少 session_id!" -ForegroundColor Red
    }
} else {
    Write-Host "       登录失败, 后续接口将返回 401" -ForegroundColor Red
}

Write-Host "==================== 3. 旧测试接口 ====================" -ForegroundColor Cyan
Invoke-Api -Method GET -Path "/get?name=test&page=1"
Invoke-Api -Method POST -Path "/post" -Body "custom_username=alice&custom_password=secret" -ContentType "application/x-www-form-urlencoded"
Invoke-Api -Method POST -Path "/getBody" -Body @{ html_code = "<b>hello</b>" }
Invoke-Api -Method POST -Path "/paginate" -Body @{ sql = "select * from category"; page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/paginate-with-params" -Body @{ sql = "select * from category where category_type = ?"; page = 1; page_size = 10; params = @(1) }

Write-Host "==================== 4. sys-dict ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/api/sys-dict/list" -Body @{ page = 1; size = 10 }
Invoke-Api -Method POST -Path "/sys-dict/select-page" -Body @{ page = 1; size = 10 }
Invoke-Api -Method POST -Path "/sys-dict/select-page" -Body @{ page = 1; size = 10; code = "dict_"; name = "字典" }
Invoke-Api -Method POST -Path "/sys-dict/insert" -Body @{ name = "测试字典"; code = "test_code"; state = 1 }
Invoke-Api -Method POST -Path "/sys-dict/update" -Body @{ id = "1"; name = "字典项1改"; code = "dict_001"; state = 1 }
Invoke-Api -Method POST -Path "/sys-dict/delete" -Body "nonexist_dict"

Write-Host "==================== 5. address-book ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/address-book/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/address-book/insert" -Body @{ user_id = "1"; consignee = "测试收货人"; phone = "13800000000"; label = "家" }
Invoke-Api -Method POST -Path "/address-book/update" -Body @{ id = "1"; user_id = "1"; consignee = "张三"; phone = "13800138000"; label = "家" }
Invoke-Api -Method POST -Path "/address-book/delete" -Body "nonexist_addr"

Write-Host "==================== 6. category ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/category/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/category/select-page" -Body @{ page = 1; page_size = 10; category_type = 1; name = "菜" }
Invoke-Api -Method POST -Path "/category/insert" -Body @{ category_type = 1; name = "测试分类"; sort = 99 }
Invoke-Api -Method POST -Path "/category/update" -Body @{ id = "1"; category_type = 1; name = "川菜"; sort = 1 }
Invoke-Api -Method POST -Path "/category/delete" -Body "nonexist_cat"

Write-Host "==================== 7. dish ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/dish/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/dish/select-page" -Body @{ page = 1; page_size = 10; name = "鸡" }
Invoke-Api -Method POST -Path "/dish/insert" -Body @{ name = "测试菜品"; category_id = "1"; price = "10.00"; code = "T001"; status = 1 }
Invoke-Api -Method POST -Path "/dish/update" -Body @{ id = "1"; name = "宫保鸡丁"; category_id = "1"; price = "38.00"; code = "D001"; status = 1 }
Invoke-Api -Method POST -Path "/dish/delete" -Body "nonexist_dish"

Write-Host "==================== 8. dish-flavor ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/dish-flavor/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/dish-flavor/insert" -Body @{ dish_id = "1"; name = "测试口味"; value = "微辣" }
Invoke-Api -Method POST -Path "/dish-flavor/update" -Body @{ id = "1"; dish_id = "1"; name = "辣度"; value = "微辣,中辣,特辣" }
Invoke-Api -Method POST -Path "/dish-flavor/delete" -Body "nonexist_flavor"

Write-Host "==================== 9. employee ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/employee/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/employee/select-page" -Body @{ page = 1; page_size = 10; name = "张" }
Invoke-Api -Method POST -Path "/employee/insert" -Body @{ name = "测试员工"; username = "test_emp"; phone = "13800000000"; sex = 1; status = 1 }
Invoke-Api -Method POST -Path "/employee/update" -Body @{ id = "1"; name = "管理员"; username = "admin"; phone = "13800138000"; sex = 1; status = 1 }
Invoke-Api -Method POST -Path "/employee/delete" -Body "nonexist_emp"

Write-Host "==================== 10. order-detail ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/order-detail/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/order-detail/insert" -Body @{ order_id = "1"; name = "测试明细"; number = 1; amount = "10.00" }
Invoke-Api -Method POST -Path "/order-detail/update" -Body @{ id = "1"; order_id = "1"; name = "宫保鸡丁"; number = 2; amount = "76.00" }
Invoke-Api -Method POST -Path "/order-detail/delete" -Body "nonexist_od"

Write-Host "==================== 11. orders ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/orders/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/orders/insert" -Body @{ number = "TEST001"; status = 1; user_id = "1"; amount = "100.00" }
Invoke-Api -Method POST -Path "/orders/update" -Body @{ id = "1"; number = "O202403190001"; status = 1; user_id = "1"; amount = "124.00" }
Invoke-Api -Method POST -Path "/orders/delete" -Body "nonexist_order"

Write-Host "==================== 12. setmeal ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/setmeal/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/setmeal/select-page" -Body @{ page = 1; page_size = 10; name = "套餐" }
Invoke-Api -Method POST -Path "/setmeal/insert" -Body @{ category_id = "1"; name = "测试套餐"; price = "88.00"; status = 1; code = "T001" }
Invoke-Api -Method POST -Path "/setmeal/update" -Body @{ id = "1"; category_id = "1"; name = "川菜套餐A"; price = "88.00"; status = 1; code = "S001" }
Invoke-Api -Method POST -Path "/setmeal/delete" -Body "nonexist_setmeal"

Write-Host "==================== 13. setmeal-dish ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/setmeal-dish/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/setmeal-dish/insert" -Body @{ setmeal_id = "1"; dish_id = "1"; name = "测试套餐菜"; price = "38.00"; copies = 1 }
Invoke-Api -Method POST -Path "/setmeal-dish/update" -Body @{ id = "1"; setmeal_id = "1"; dish_id = "1"; name = "宫保鸡丁"; price = "38.00"; copies = 1 }
Invoke-Api -Method POST -Path "/setmeal-dish/delete" -Body "nonexist_sd"

Write-Host "==================== 14. shopping-cart ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/shopping-cart/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/shopping-cart/insert" -Body @{ name = "测试购物车"; user_id = "1"; number = 1; amount = "10.00" }
Invoke-Api -Method POST -Path "/shopping-cart/update" -Body @{ id = "1"; name = "宫保鸡丁"; user_id = "1"; number = 2; amount = "76.00" }
Invoke-Api -Method POST -Path "/shopping-cart/delete" -Body "nonexist_cart"

Write-Host "==================== 15. user ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/user/select-page" -Body @{ page = 1; page_size = 10 }
Invoke-Api -Method POST -Path "/user/select-page" -Body @{ page = 1; page_size = 10; name = "用户" }
Invoke-Api -Method POST -Path "/user/insert" -Body @{ openid = "test_openid"; name = "测试用户"; phone = "13800000000"; sex = 1 }
Invoke-Api -Method POST -Path "/user/update" -Body @{ id = "1"; openid = "openid_001"; name = "用户1"; phone = "13800138001"; sex = 1 }
Invoke-Api -Method POST -Path "/user/delete" -Body "nonexist_user"

Write-Host "==================== 16. 报表接口 ====================" -ForegroundColor Cyan
Invoke-Api -Method GET -Path "/report/order-details"
Invoke-Api -Method GET -Path "/report/setmeal-dishes"
Invoke-Api -Method GET -Path "/report/shopping-cart"
Invoke-Api -Method GET -Path "/report/dish-full-info"
Invoke-Api -Method GET -Path "/report/user-order-stats"
Invoke-Api -Method GET -Path "/report/address-book"

Write-Host "==================== 17. 登出 ====================" -ForegroundColor Cyan
Invoke-Api -Method POST -Path "/api/logout"

Write-Host "`n==================== 汇总 ====================" -ForegroundColor Cyan
$fail = $results | Where-Object { -not $_.Pass }
$pass = $results | Where-Object { $_.Pass }
Write-Host ("总计: {0}  PASS: {1}  FAIL: {2}" -f $results.Count, $pass.Count, $fail.Count) -ForegroundColor $(if ($fail.Count -eq 0) { "Green" } else { "Yellow" })
if ($fail.Count -gt 0) {
    Write-Host "`n失败明细:" -ForegroundColor Red
    $fail | Format-Table Method, Path, Status, Body -AutoSize | Out-String | Write-Host
}
