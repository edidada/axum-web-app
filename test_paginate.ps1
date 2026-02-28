# 登录获取 session token
Write-Host "Login request..."
$loginResponse = Invoke-RestMethod -Uri 'http://localhost:9001/login' -Method POST -Headers @{'Content-Type'='application/json'} -Body '{"username": "user", "password": "password"}'
Write-Host "Login response: $loginResponse"

# 从 HTML 响应中提取 session ID
$token = $loginResponse -replace '.*Logged in with session ID: (.*)', '$1'
Write-Host "Extracted token: $token"

# 使用 token 访问分页接口
Write-Host "Paginate request with token: $token"
Invoke-RestMethod -Uri 'http://localhost:9001/paginate' -Method POST -Headers @{'Content-Type'='application/json'; 'Authorization'="Bearer $token"} -Body '{"sql": "SELECT * FROM users", "page": 1, "page_size": 10}'