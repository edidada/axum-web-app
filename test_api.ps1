# 登录获取 session ID
$loginResponse = Invoke-WebRequest -Uri "http://localhost:9001/login" -Method POST -Headers @{"Content-Type"="application/json"} -Body '{"username": "user", "password": "password"}'

# 提取 session ID
$sessionId = $loginResponse.Content -replace "Logged in with session ID: ", ""

Write-Host "Session ID: $sessionId"

# 测试原有的分页接口
Write-Host "\nTesting original paginate endpoint:"
$paginateResponse = Invoke-WebRequest -Uri "http://localhost:9001/paginate" -Method POST -Headers @{"Content-Type"="application/json"; "Authorization"="Bearer $sessionId"} -Body '{"sql": "SELECT * FROM users", "page": 1, "page_size": 10}'
Write-Host $paginateResponse.Content

# 测试新的带参数的分页接口
Write-Host "\nTesting new paginate-with-params endpoint:"
$paginateWithParamsResponse = Invoke-WebRequest -Uri "http://localhost:9001/paginate-with-params" -Method POST -Headers @{"Content-Type"="application/json"; "Authorization"="Bearer $sessionId"} -Body '{"sql": "SELECT * FROM users WHERE age > ?", "page": 1, "page_size": 10, "params": [18]}'
Write-Host $paginateWithParamsResponse.Content
