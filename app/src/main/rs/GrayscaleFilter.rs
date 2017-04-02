#pragma version(1)
#pragma rs java_package_name(com.vinay.imagefilter.renderscript)

#include "Clamp.rsh"
#include "YUV.rsh"

// set from the java SDK level
rs_allocation gIn;
rs_allocation gOut;
rs_script gScript;

// Magic factors

// Static variables

static void setup() {
}

void filter() {
	setup();
    rsForEach(gScript, gIn, gOut, 0, 0);	// for each element of the input allocation,
    										// call root() method on gScript
}

void root(const uchar4 *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {
	float4 f4 = rsUnpackColor8888(*v_in);	// extract RGBA values, see rs_core.rsh
	
    float grayscale = GetGrayscale(f4.rgb);
    
    float3 f3 = {grayscale, grayscale, grayscale};
    
    *v_out = rsPackColorTo8888(f3);
}