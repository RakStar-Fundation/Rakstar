build:
	cargo build --target i686-unknown-linux-gnu 
	cp target/i686-unknown-linux-gnu/debug/librakstar_gdk.so Server/components/rakstar.so
run:
	./Server/omp-server

