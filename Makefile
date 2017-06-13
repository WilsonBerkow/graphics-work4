compile: clean main

main:
	rustc -C opt-level=3 src/main.rs

d: clean
	rustc src/main.rs

run:
	./main
	if [ -f anim/football.gif ]; then rm anim/football.gif; fi
	convert anim/*.png anim/football.gif

clean:
	rm -f main
