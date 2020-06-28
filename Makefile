android-build_debug:
	cd app/android && cargo build --target aarch64-linux-android
	#cd app/android && cargo build --target armv7-linux-androideabi
	# TODO:
	#cd app/android && cargo build --target i686-linux-android

	cp target/aarch64-linux-android/debug/libpixel_game.so app/android/project/app/src/main/jniLibs/arm64-v8a/libmain.so
	#cp target/armv7-linux-androideabi/debug/libpixel_game.so app/android/project/app/src/main/jniLibs/armeabi-v7a/libmain.so
    # TODO:
	#cp target/i686-linux-android/debug/libpixel_game.so app/android/project/app/src/main/jniLibs/x86/libmain.so

android-run: android-build_debug
