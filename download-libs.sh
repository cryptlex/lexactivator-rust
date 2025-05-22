# Exit on error
set -e

BASE_URL=https://dl.cryptlex.com/downloads
VERSION="v3.32.0";

mkdir -p tmp/windows
WINDOWS_FILE_NAME="LexActivator-Win.zip"
curl -O ${BASE_URL}/${VERSION}/${WINDOWS_FILE_NAME}
unzip ${WINDOWS_FILE_NAME} -d ./tmp/windows
# x64
cp ./tmp/windows/libs/vc14/x64/LexActivator.dll ./libs/win32-x86_64/
cp ./tmp/windows/libs/vc14/x64/LexActivator.lib ./libs/win32-x86_64/

# x86
cp ./tmp/windows/libs/vc14/x86/LexActivator.dll ./libs/win32-x86/
cp ./tmp/windows/libs/vc14/x86/LexActivator.lib ./libs/win32-x86/

mkdir -p tmp/macos
MAC_FILE_NAME="LexActivator-Static-Mac.zip"
curl -O ${BASE_URL}/${VERSION}/${MAC_FILE_NAME}
unzip ${MAC_FILE_NAME} -d ./tmp/macos
# Darwin x64
cp ./tmp/macos/libs/clang/x86_64/libLexActivator.a ./libs/darwin-x86_64/
# Darwin ARM64
cp ./tmp/macos/libs/clang/arm64/libLexActivator.a ./libs/darwin-aarch64/

mkdir -p tmp/linux
LINUX_FILE_NAME="LexActivator-Static-Linux.zip"
curl -O ${BASE_URL}/${VERSION}/${LINUX_FILE_NAME}
unzip ${LINUX_FILE_NAME} -d ./tmp/linux
# GCC6 x64
cp ./tmp/linux/libs/gcc-6/amd64/libLexActivator.a ./libs/linux-x86_64/
# GCC6 x86
cp ./tmp/linux/libs/gcc-6/i386/libLexActivator.a ./libs/linux-x86/
# GCC6 ARM64
cp ./tmp/linux/libs/gcc-6/arm64/libLexActivator.a ./libs/linux-aarch64/
# MUSL ARM64
cp ./tmp/linux/libs/musl/arm64/libLexActivator.a ./libs/musl-aarch64/
# MUSL x64
cp ./tmp/linux/libs/musl/amd64/libLexActivator.a ./libs/musl-x86_64/
# cp ./tmp/linux/libs/gcc/armhf/libLexActivator.so ./linux-arm/

rm -f ${WINDOWS_FILE_NAME}
rm -f ${MAC_FILE_NAME}
rm -f ${LINUX_FILE_NAME}
rm -R -f ./tmp