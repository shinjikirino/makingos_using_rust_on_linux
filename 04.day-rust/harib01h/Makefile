OSNAME=os

ASRC=./src/asm
CSRC=./src/c
RSRC=./src/rust
OBJ=./obj
LS=./ls
ENTRY=Main

IMG=$(OSNAME).img
OSSYS=$(OBJ)/$(OSNAME).sys
IPL=$(OBJ)/ipl.bin

RUSTCOPT=--target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C opt-level=2 -C no-prepopulate-passes -C no-stack-check -Z verbose -Z no-landing-pads -C relocation-model=static -L$(OBJ)  
# -C relocation-model=static を追加。これがないと、bootpack.rsにexternで書いた関数が位置独立コードとしてGOTを参照しにいって、GOTが定義されていないエラーとなる。今回はfunc.sに関数の中身がいるので、静的に再配置させるようコンパイラに認識させる
BINOPT=-nostdlib -Wl,--oformat=binary
QEMUOPT=-m 32 -localtime -vga std -fda
ASOPT=--32 -march=i386
LDOPT=-m elf_i386 -nostdlib --oformat=binary

$(IMG) : $(OSSYS) $(IPL)
	mformat -f 1440 -C -B $(IPL) -i $(IMG) ::
	mcopy $(OSSYS) -i $(IMG) ::

$(OSSYS) : $(ASRC)/head.s $(ASRC)/func.s $(RSRC)/bootpack.rs
	gcc -m32 $(ASRC)/head.s -nostdlib -T$(LS)/head.ls -o $(OBJ)/head.bin
	rustc $(RUSTCOPT) -o $(OBJ)/boot.o $(RSRC)/*.rs  
	as $(ASOPT) $(ASRC)/func.s -o $(OBJ)/func.o
	ld $(LDOPT) -e $(ENTRY) -T$(LS)/boot.ls -o $(OBJ)/boot.bin $(OBJ)/boot.o $(OBJ)/func.o
	cat $(OBJ)/head.bin $(OBJ)/boot.bin > $(OSSYS)

$(IPL) : $(ASRC)/ipl.s
	gcc $(ASRC)/ipl.s -nostdlib -T$(LS)/ipl.ls -o $(IPL)

run		: $(IMG)
	qemu-system-i386 $(QEMUOPT) $(IMG)

debug	: $(IMG)
	qemu-system-i386 -s -S $(QEMUOPT) $(IMG) -redir tcp:5555:127.0.0.1:1234 &

img		:;	make $(IMG)

clean	:;	rm $(OBJ)/*
