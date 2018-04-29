OSNAME=os

ASRC=./src/asm
CSRC=./src/c
RSRC=./src/rust
OBJ=./obj
LS=./ls

IMG=$(OSNAME).img
OSSYS=$(OBJ)/$(OSNAME).sys
IPL=$(OBJ)/ipl.bin

BINOPT=-nostdlib -Wl,--oformat=binary
QEMUOPT=-m 32 -localtime -vga std -fda

$(IMG) : $(OSSYS) $(IPL)
	mformat -f 1440 -C -B $(IPL) -i $(IMG) ::
	mcopy $(OSSYS) -i $(IMG) ::

$(OSSYS) : $(ASRC)/head.s $(ASRC)/func.s $(RSRC)/bootpack.rs
	gcc $(ASRC)/head.s -nostdlib -T$(LS)/head.ls -o $(OBJ)/head.bin
	rustc --crate-type=staticlib --emit=obj -C lto -C opt-level=2 -C no-prepopulate-passes -C no-stack-check -Z verbose -Z no-landing-pads -o $(OBJ)/boot.o $(RSRC)/*.rs
	as $(ASRC)/func.s -o $(OBJ)/func.o
	ld -o $(OBJ)/boot.bin -e Main --oformat=binary $(OBJ)/boot.o $(OBJ)/func.o
	cat $(OBJ)/head.bin $(OBJ)/boot.bin > $(OSSYS)

$(IPL) : $(ASRC)/ipl.s
	gcc $(ASRC)/ipl.s -nostdlib -T$(LS)/ipl.ls -o $(IPL)

run		: $(IMG)
	qemu-system-i386 $(QEMUOPT) $(IMG)

debug	: $(IMG)
	qemu-system-i386 -s -S $(QEMUOPT) $(IMG) -redir tcp:5555:127.0.0.1:1234 &

img		:;	make $(IMG)

clean	:;	rm $(OBJ)/*
