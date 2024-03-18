install:
	cargo install --path .

uninstall:
	cargo uninstall euchre-tui

record:
	vhs assets/record.tape
