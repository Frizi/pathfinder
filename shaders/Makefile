TARGET_DIR?=../resources/shaders

EMPTY=

SHADERS=\
	debug_solid.fs.glsl \
	debug_solid.vs.glsl \
	debug_texture.fs.glsl \
	debug_texture.vs.glsl \
	demo_ground.fs.glsl \
	demo_ground.vs.glsl \
	fill.fs.glsl \
	fill.vs.glsl \
	post.fs.glsl \
	post.vs.glsl \
	reproject.fs.glsl \
	reproject.vs.glsl \
	stencil.fs.glsl \
	stencil.vs.glsl \
	tile_alpha.fs.glsl \
	tile_alpha.vs.glsl \
	tile_solid.fs.glsl \
	tile_solid.vs.glsl \
	$(EMPTY)

INCLUDES=\
	post_convolve.inc.glsl \
	post_gamma_correct.inc.glsl \
	$(EMPTY)

OUT=\
	$(SHADERS:%=$(TARGET_DIR)/gl3/%) \
	$(SHADERS:%.glsl=$(TARGET_DIR)/metal/%.metal) \
	$(SHADERS:%.glsl=build/metal/%.spv) \
	$(EMPTY)

GLSL_VERSION=330
GLSLANGFLAGS=--auto-map-locations -I.
GLSLANGFLAGS_METAL=$(GLSLANGFLAGS) -DPF_ORIGIN_UPPER_LEFT=1

SPIRVCROSS?=spirv-cross
SPIRVCROSSFLAGS=--msl --msl-version 020100 --msl-argument-buffers

GLSL_VERSION_HEADER="\#version {{version}}"
HEADER="// Automatically generated from files in pathfinder/shaders/. Do not edit!"

GLSL_SED_ARGS=-e "s/\#version 330//" -e "s/\#line.*$$//"

all:	$(OUT)

.PHONY: clean

clean:
	rm -f $(OUT)

test:
	echo $(GLSL_SED_ARGS) 
	
build/metal/%.fs.spv:	%.fs.glsl $(INCLUDES)
	mkdir -p build/metal && glslangValidator $(GLSLANGFLAGS_METAL) -G$(GLSL_VERSION) -S frag -o $@ $<

$(TARGET_DIR)/gl3/%.fs.glsl:	%.fs.glsl $(INCLUDES)
	mkdir -p $(TARGET_DIR)/gl3 && echo $(GLSL_VERSION_HEADER) > $@ && echo $(HEADER) >> $@ && ( glslangValidator $(GLSLANGFLAGS) -S frag -E $< | sed $(GLSL_SED_ARGS) >> $@ ) || ( rm $@ && exit 1 )

build/metal/%.vs.spv:	%.vs.glsl $(INCLUDES)
	mkdir -p build/metal && glslangValidator $(GLSLANGFLAGS_METAL) -G$(GLSL_VERSION) -S vert -o $@ $<

$(TARGET_DIR)/gl3/%.vs.glsl:	%.vs.glsl $(INCLUDES)
	mkdir -p $(TARGET_DIR)/gl3 && echo $(GLSL_VERSION_HEADER) > $@ && echo $(HEADER) >> $@ && ( glslangValidator $(GLSLANGFLAGS) -S vert -E $< | sed $(GLSL_SED_ARGS) >> $@ ) || ( rm $@ && exit 1 )

$(TARGET_DIR)/metal/%.metal:	build/metal/%.spv
	mkdir -p $(TARGET_DIR)/metal && echo $(HEADER) > $@ && ( $(SPIRVCROSS) $(SPIRVCROSSFLAGS) $< >> $@ ) || ( rm $@ && exit 1 )
