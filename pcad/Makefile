all: evict_demo.c evict.c detect.c victim.c
	gcc evict_demo.c -o evict_demo -lpthread -O0
	gcc evict.c -o evict -lpthread -O0
	gcc detect.c -o detect
	gcc victim.c -o victim

clean:
	rm -f evict_demo detect victim evict