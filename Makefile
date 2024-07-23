.PHONY: build_mac

build_mac:
	cargo build --bin vinseers-gui -r

	mkdir -p target/vinseers_DMG/vinseers.app/Contents/MacOS
	mkdir -p target/vinseers_DMG/vinseers.app/Contents/Resources
	cp mac/Info.plist target/vinseers_DMG/vinseers.app/Contents
	cp target/release/vinseers-gui target/vinseers_DMG/vinseers.app/Contents/MacOS/vinseers

	hdiutil create -volname "vinseers.app" -srcfolder target/vinseers_DMG -ov -format UDZO vinseers.dmg

