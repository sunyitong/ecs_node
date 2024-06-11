@echo off
echo Starting build process...

:: Cross编译项目
cross build --release

:: 检查编译是否成功
if %ERRORLEVEL% neq 0 (
    echo Build failed, stopping script.
    exit /b %ERRORLEVEL%
)

echo Build successful, deploying to device...

:: 定义二进制文件路径
set BIN_PATH=./target/armv7-unknown-linux-gnueabihf/release/ecs_node
set TARGET_PATH=/root/ecsnode

:: 将二进制文件推送到设备
adb push %BIN_PATH% %TARGET_PATH%

:: 检查 adb push 命令是否成功
if %ERRORLEVEL% neq 0 (
    echo Failed to push binary to device.
    exit /b %ERRORLEVEL%
)

echo Setting execute permission...

:: 更改文件权限
adb shell "sudo chmod +x %TARGET_PATH%"

:: 检查 chmod 命令是否成功
if %ERRORLEVEL% neq 0 (
    echo Failed to set execute permissions.
    exit /b %ERRORLEVEL%
)

echo Running the binary...

:: 运行二进制文件
adb shell "sudo %TARGET_PATH%"

:: 检查运行命令是否成功
if %ERRORLEVEL% neq 0 (
    echo Failed to run the binary on the device.
    exit /b %ERRORLEVEL%
)

echo Process completed successfully!
