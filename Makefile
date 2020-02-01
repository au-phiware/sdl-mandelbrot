out ?= result
name := mandelbrot

.PHONY: check install installcheck

$(name): src/main.c
	gcc $(shell sdl2-config --cflags --libs) -o $@ $<

check:
	stat $(name)

install: $(out)/$(name)

installcheck:
	stat $(out)/$(name)

$(out)/$(name): $(name)
	mkdir -p $(@D)
	install $< $(@D)
