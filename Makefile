LIBNAME = aircraft_sim

debug: clean
	cargo build
	cp ./target/debug/lib${LIBNAME}.so ./run/${LIBNAME}.so
	cp ./src/main.py ./run/main.py

release: clean
	cargo build --release
	cp ./target/release/lib${LIBNAME}.so ./run/${LIBNAME}.so
	cp ./src/main.py ./run/main.py

clean:
	- rm ./run/*

run: debug
	python3 ./run/main.py
