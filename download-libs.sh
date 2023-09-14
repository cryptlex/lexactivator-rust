BASE_URL=https://dl.cryptlex.com/downloads
VERSION="v3.23.1";

mkdir -p tmp/windows
WINDOWS_FILE_NAME="LexActivator-Static-Win-VC14.zip"
curl -O ${BASE_URL}/${VERSION}/${WINDOWS_FILE_NAME}
unzip ${WINDOWS_FILE_NAME} -d ./tmp/windows
cp ./tmp/windows/libs/vc14/x64/LexActivator.lib ./libs/win32-x86-64/
cp ./tmp/windows/libs/vc14/x86/LexActivator.lib ./libs/win32-x86/

mkdir -p tmp/macos
MAC_FILE_NAME="LexActivator-Static-Mac.zip"
curl -O ${BASE_URL}/${VERSION}/${MAC_FILE_NAME}
unzip ${MAC_FILE_NAME} -d ./tmp/macos
cp ./tmp/macos/libs/clang/x86_64/libLexActivator.a ./libs/darwin-x86-64/
cp ./tmp/macos/libs/clang/arm64/libLexActivator.a ./libs/darwin-aarch64/

mkdir -p tmp/linux
LINUX_FILE_NAME="LexActivator-Static-Linux.zip"
curl -O ${BASE_URL}/${VERSION}/${LINUX_FILE_NAME}
unzip ${LINUX_FILE_NAME} -d ./tmp/linux
# GCC6
cp ./tmp/linux/libs/gcc-6/amd64/libLexActivator.a ./libs/linux-x86-64/
cp ./tmp/linux/libs/gcc-6/i386/libLexActivator.a ./libs/linux-x86/
cp ./tmp/linux/libs/gcc-6/arm64/libLexActivator.a ./libs/linux-aarch64/
# MUSL
cp ./tmp/linux/libs/musl/arm64/libLexActivator.a ./libs/musl-aarch64/
cp ./tmp/linux/libs/musl/amd64/libLexActivator.a ./libs/musl-x86-64/
# cp ./tmp/linux/libs/gcc/armhf/libLexActivator.so ./linux-arm/

rm -f ${WINDOWS_FILE_NAME}
rm -f ${MAC_FILE_NAME}
rm -f ${LINUX_FILE_NAME}
rm -R -f ./tmp