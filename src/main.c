#include <stdio.h>
#include <SDL2/SDL.h>

#define SUCCESS 0

#define DIV_LIMIT 200

static Uint64 frameClockSamples[64] = {0};
static int currentFrameClockSample = 1;
static void TrackFrameRate() {
    frameClockSamples[currentFrameClockSample++ & 63] = SDL_GetPerformanceCounter();
    if (((currentFrameClockSample & 63) == 1) && (frameClockSamples[0] != 0)) {
        Uint64 ticks = frameClockSamples[0] - frameClockSamples[1];
        SDL_LogInfo(SDL_LOG_CATEGORY_APPLICATION, "%5.1f fps", (float)SDL_GetPerformanceFrequency()/(ticks>>6));
    }
}

// Palette used by [Sonic Robo Blast 2](https://www.srb2.org/).
const SDL_Color colors[256] = {
    /*   0 */ { 0xff, 0xff, 0xff, 0xFF },
    /*   1 */ { 0xf7, 0xf7, 0xf7, 0xFF },
    /*   2 */ { 0xef, 0xef, 0xef, 0xFF },
    /*   3 */ { 0xe7, 0xe7, 0xe7, 0xFF },
    /*   4 */ { 0xdf, 0xdf, 0xdf, 0xFF },
    /*   5 */ { 0xd7, 0xd7, 0xd7, 0xFF },
    /*   6 */ { 0xcf, 0xcf, 0xcf, 0xFF },
    /*   7 */ { 0xc7, 0xc7, 0xc7, 0xFF },
    /*   8 */ { 0xbf, 0xbf, 0xbf, 0xFF },
    /*   9 */ { 0xb7, 0xb7, 0xb7, 0xFF },
    /*  10 */ { 0xaf, 0xaf, 0xaf, 0xFF },
    /*  11 */ { 0xa7, 0xa7, 0xa7, 0xFF },
    /*  12 */ { 0x9f, 0x9f, 0x9f, 0xFF },
    /*  13 */ { 0x97, 0x97, 0x97, 0xFF },
    /*  14 */ { 0x8f, 0x8f, 0x8f, 0xFF },
    /*  15 */ { 0x87, 0x87, 0x87, 0xFF },
    /*  16 */ { 0x7f, 0x7f, 0x7f, 0xFF },
    /*  17 */ { 0x77, 0x77, 0x77, 0xFF },
    /*  18 */ { 0x6f, 0x6f, 0x6f, 0xFF },
    /*  19 */ { 0x67, 0x67, 0x67, 0xFF },
    /*  20 */ { 0x5f, 0x5f, 0x5f, 0xFF },
    /*  21 */ { 0x57, 0x57, 0x57, 0xFF },
    /*  22 */ { 0x4f, 0x4f, 0x4f, 0xFF },
    /*  23 */ { 0x47, 0x47, 0x47, 0xFF },
    /*  24 */ { 0x3f, 0x3f, 0x3f, 0xFF },
    /*  25 */ { 0x37, 0x37, 0x37, 0xFF },
    /*  26 */ { 0x2f, 0x2f, 0x2f, 0xFF },
    /*  27 */ { 0x27, 0x27, 0x27, 0xFF },
    /*  28 */ { 0x1f, 0x1f, 0x1f, 0xFF },
    /*  29 */ { 0x17, 0x17, 0x17, 0xFF },
    /*  30 */ { 0x0f, 0x0f, 0x0f, 0xFF },
    /*  31 */ { 0x07, 0x07, 0x07, 0xFF },
    /*  32 */ { 0x00, 0x00, 0x00, 0xFF }, // black
    /*  33 */ { 0xbf, 0xa7, 0x8f, 0xFF },
    /*  34 */ { 0xb7, 0xa0, 0x88, 0xFF },
    /*  35 */ { 0xaf, 0x98, 0x80, 0xFF },
    /*  36 */ { 0xa7, 0x90, 0x78, 0xFF },
    /*  37 */ { 0x9f, 0x89, 0x71, 0xFF },
    /*  38 */ { 0x96, 0x81, 0x69, 0xFF },
    /*  39 */ { 0x8e, 0x79, 0x61, 0xFF },
    /*  40 */ { 0x86, 0x72, 0x5a, 0xFF },
    /*  41 */ { 0x7e, 0x6a, 0x52, 0xFF },
    /*  42 */ { 0x75, 0x62, 0x4a, 0xFF },
    /*  43 */ { 0x6d, 0x5a, 0x42, 0xFF },
    /*  44 */ { 0x65, 0x53, 0x3b, 0xFF },
    /*  45 */ { 0x5d, 0x4b, 0x33, 0xFF },
    /*  46 */ { 0x54, 0x43, 0x2b, 0xFF },
    /*  47 */ { 0x4c, 0x3c, 0x24, 0xFF },
    /*  48 */ { 0x43, 0x33, 0x1b, 0xFF },
    /*  49 */ { 0xbf, 0x7b, 0x4b, 0xFF },
    /*  50 */ { 0xb3, 0x73, 0x47, 0xFF },
    /*  51 */ { 0xab, 0x6f, 0x43, 0xFF },
    /*  52 */ { 0xa3, 0x6b, 0x3f, 0xFF },
    /*  53 */ { 0x9b, 0x63, 0x3b, 0xFF },
    /*  54 */ { 0x8f, 0x5f, 0x37, 0xFF },
    /*  55 */ { 0x87, 0x57, 0x33, 0xFF },
    /*  56 */ { 0x7f, 0x53, 0x2f, 0xFF },
    /*  57 */ { 0x77, 0x4f, 0x2b, 0xFF },
    /*  58 */ { 0x6b, 0x47, 0x27, 0xFF },
    /*  59 */ { 0x5f, 0x43, 0x23, 0xFF },
    /*  60 */ { 0x53, 0x3f, 0x1f, 0xFF },
    /*  61 */ { 0x4b, 0x37, 0x1b, 0xFF },
    /*  62 */ { 0x3f, 0x2f, 0x17, 0xFF },
    /*  63 */ { 0x33, 0x2b, 0x13, 0xFF },
    /*  64 */ { 0x2b, 0x23, 0x0f, 0xFF },
    /*  65 */ { 0xff, 0xeb, 0xdf, 0xFF },
    /*  66 */ { 0xff, 0xe3, 0xd3, 0xFF },
    /*  67 */ { 0xff, 0xdb, 0xc7, 0xFF },
    /*  68 */ { 0xff, 0xd3, 0xbb, 0xFF },
    /*  69 */ { 0xff, 0xcf, 0xb3, 0xFF },
    /*  70 */ { 0xff, 0xc7, 0xa7, 0xFF },
    /*  71 */ { 0xff, 0xbf, 0x9b, 0xFF },
    /*  72 */ { 0xff, 0xbb, 0x93, 0xFF },
    /*  73 */ { 0xff, 0xb3, 0x83, 0xFF },
    /*  74 */ { 0xf7, 0xab, 0x7b, 0xFF },
    /*  75 */ { 0xef, 0xa3, 0x73, 0xFF },
    /*  76 */ { 0xe7, 0x9b, 0x6b, 0xFF },
    /*  77 */ { 0xdf, 0x93, 0x63, 0xFF },
    /*  78 */ { 0xd7, 0x8b, 0x5b, 0xFF },
    /*  79 */ { 0xcf, 0x83, 0x53, 0xFF },
    /*  80 */ { 0xcb, 0x7f, 0x4f, 0xFF },
    /*  81 */ { 0xff, 0xee, 0xdc, 0xFF },
    /*  82 */ { 0xff, 0xdc, 0xb9, 0xFF },
    /*  83 */ { 0xff, 0xcb, 0x97, 0xFF },
    /*  84 */ { 0xff, 0xb9, 0x75, 0xFF },
    /*  85 */ { 0xff, 0xa8, 0x55, 0xFF },
    /*  86 */ { 0xff, 0x97, 0x36, 0xFF },
    /*  87 */ { 0xff, 0x86, 0x19, 0xFF },
    /*  88 */ { 0xff, 0x75, 0x00, 0xFF },
    /*  89 */ { 0xf3, 0x6d, 0x00, 0xFF },
    /*  90 */ { 0xe5, 0x65, 0x00, 0xFF },
    /*  91 */ { 0xd8, 0x5d, 0x00, 0xFF },
    /*  92 */ { 0xcb, 0x55, 0x00, 0xFF },
    /*  93 */ { 0xbe, 0x4d, 0x00, 0xFF },
    /*  94 */ { 0xb1, 0x45, 0x00, 0xFF },
    /*  95 */ { 0xa4, 0x3d, 0x00, 0xFF },
    /*  96 */ { 0x97, 0x36, 0x00, 0xFF },
    /*  97 */ { 0xff, 0xff, 0xef, 0xFF },
    /*  98 */ { 0xff, 0xff, 0xcf, 0xFF },
    /*  99 */ { 0xff, 0xff, 0xaf, 0xFF },
    /* 100 */ { 0xff, 0xff, 0x8f, 0xFF },
    /* 101 */ { 0xff, 0xff, 0x6f, 0xFF },
    /* 102 */ { 0xff, 0xff, 0x4f, 0xFF },
    /* 103 */ { 0xff, 0xff, 0x2f, 0xFF },
    /* 104 */ { 0xff, 0xff, 0x0f, 0xFF },
    /* 105 */ { 0xff, 0xff, 0x00, 0xFF },
    /* 106 */ { 0xcf, 0xcf, 0x00, 0xFF },
    /* 107 */ { 0xaf, 0xaf, 0x00, 0xFF },
    /* 108 */ { 0x8f, 0x8f, 0x00, 0xFF },
    /* 109 */ { 0x6f, 0x6f, 0x00, 0xFF },
    /* 110 */ { 0x4f, 0x4f, 0x00, 0xFF },
    /* 111 */ { 0x2f, 0x2f, 0x00, 0xFF },
    /* 112 */ { 0x0f, 0x0f, 0x00, 0xFF },
    /* 113 */ { 0xff, 0xff, 0x73, 0xFF },
    /* 114 */ { 0xeb, 0xdb, 0x57, 0xFF },
    /* 115 */ { 0xd7, 0xbb, 0x43, 0xFF },
    /* 116 */ { 0xc3, 0x9b, 0x2f, 0xFF },
    /* 117 */ { 0xaf, 0x7b, 0x1f, 0xFF },
    /* 118 */ { 0x9b, 0x5b, 0x13, 0xFF },
    /* 119 */ { 0x87, 0x43, 0x07, 0xFF },
    /* 120 */ { 0x73, 0x2b, 0x00, 0xFF },
    /* 121 */ { 0xff, 0xdf, 0xdf, 0xFF },
    /* 122 */ { 0xff, 0xbf, 0xbf, 0xFF },
    /* 123 */ { 0xff, 0x9f, 0x9f, 0xFF },
    /* 124 */ { 0xff, 0x7f, 0x7f, 0xFF },
    /* 125 */ { 0xff, 0x5f, 0x5f, 0xFF },
    /* 126 */ { 0xff, 0x3f, 0x3f, 0xFF },
    /* 127 */ { 0xff, 0x1f, 0x1f, 0xFF },
    /* 128 */ { 0xff, 0x00, 0x00, 0xFF },
    /* 129 */ { 0xef, 0x00, 0x00, 0xFF },
    /* 130 */ { 0xdf, 0x00, 0x00, 0xFF },
    /* 131 */ { 0xcf, 0x00, 0x00, 0xFF },
    /* 132 */ { 0xbf, 0x00, 0x00, 0xFF },
    /* 133 */ { 0xaf, 0x00, 0x00, 0xFF },
    /* 134 */ { 0x9f, 0x00, 0x00, 0xFF },
    /* 135 */ { 0x8f, 0x00, 0x00, 0xFF },
    /* 136 */ { 0x7f, 0x00, 0x00, 0xFF },
    /* 137 */ { 0x6f, 0x00, 0x00, 0xFF },
    /* 138 */ { 0x5f, 0x00, 0x00, 0xFF },
    /* 139 */ { 0x4f, 0x00, 0x00, 0xFF },
    /* 140 */ { 0x3f, 0x00, 0x00, 0xFF },
    /* 141 */ { 0x2f, 0x00, 0x00, 0xFF },
    /* 142 */ { 0x1f, 0x00, 0x00, 0xFF },
    /* 143 */ { 0x0f, 0x00, 0x00, 0xFF },
    /* 144 */ { 0xff, 0xb7, 0xb7, 0xFF },
    /* 145 */ { 0xf3, 0xa3, 0xa3, 0xFF },
    /* 146 */ { 0xe7, 0x8f, 0x8f, 0xFF },
    /* 147 */ { 0xdb, 0x7b, 0x7b, 0xFF },
    /* 148 */ { 0xcb, 0x6b, 0x6b, 0xFF },
    /* 149 */ { 0xbf, 0x5b, 0x5b, 0xFF },
    /* 150 */ { 0xb3, 0x4f, 0x4f, 0xFF },
    /* 151 */ { 0xa7, 0x3f, 0x3f, 0xFF },
    /* 152 */ { 0x8e, 0x2e, 0x00, 0xFF },
    /* 153 */ { 0x86, 0x27, 0x00, 0xFF },
    /* 154 */ { 0x7e, 0x20, 0x00, 0xFF },
    /* 155 */ { 0x75, 0x19, 0x00, 0xFF },
    /* 156 */ { 0x6d, 0x12, 0x00, 0xFF },
    /* 157 */ { 0x65, 0x0b, 0x00, 0xFF },
    /* 158 */ { 0x5d, 0x05, 0x00, 0xFF },
    /* 159 */ { 0x55, 0x00, 0x00, 0xFF },
    /* 160 */ { 0x77, 0xff, 0x4f, 0xFF },
    /* 161 */ { 0x70, 0xf0, 0x4b, 0xFF },
    /* 162 */ { 0x69, 0xe0, 0x46, 0xFF },
    /* 163 */ { 0x61, 0xd0, 0x41, 0xFF },
    /* 164 */ { 0x5a, 0xc0, 0x3c, 0xFF },
    /* 165 */ { 0x52, 0xb0, 0x37, 0xFF },
    /* 166 */ { 0x4b, 0xa0, 0x32, 0xFF },
    /* 167 */ { 0x43, 0x90, 0x2d, 0xFF },
    /* 168 */ { 0x3c, 0x80, 0x28, 0xFF },
    /* 169 */ { 0x35, 0x70, 0x23, 0xFF },
    /* 170 */ { 0x2d, 0x60, 0x1e, 0xFF },
    /* 171 */ { 0x26, 0x50, 0x19, 0xFF },
    /* 172 */ { 0x1e, 0x40, 0x14, 0xFF },
    /* 173 */ { 0x17, 0x30, 0x0f, 0xFF },
    /* 174 */ { 0x0f, 0x20, 0x0a, 0xFF },
    /* 175 */ { 0x07, 0x0f, 0x04, 0xFF },
    /* 176 */ { 0xde, 0xff, 0xa8, 0xFF },
    /* 177 */ { 0xc7, 0xe4, 0x94, 0xFF },
    /* 178 */ { 0xad, 0xc8, 0x80, 0xFF },
    /* 179 */ { 0x95, 0xad, 0x6b, 0xFF },
    /* 180 */ { 0x7c, 0x92, 0x58, 0xFF },
    /* 181 */ { 0x64, 0x77, 0x44, 0xFF },
    /* 182 */ { 0x4a, 0x5a, 0x30, 0xFF },
    /* 183 */ { 0x32, 0x3f, 0x1d, 0xFF },
    /* 184 */ { 0x00, 0xff, 0x00, 0xFF },
    /* 185 */ { 0x00, 0xdf, 0x00, 0xFF },
    /* 186 */ { 0x00, 0xbf, 0x00, 0xFF },
    /* 187 */ { 0x00, 0x9f, 0x00, 0xFF },
    /* 188 */ { 0x00, 0x7f, 0x00, 0xFF },
    /* 189 */ { 0x00, 0x5f, 0x00, 0xFF },
    /* 190 */ { 0x00, 0x3f, 0x00, 0xFF },
    /* 191 */ { 0x00, 0x1f, 0x00, 0xFF },
    /* 192 */ { 0xff, 0x6f, 0xff, 0xFF },
    /* 193 */ { 0xff, 0x00, 0xff, 0xFF },
    /* 194 */ { 0xdf, 0x00, 0xdf, 0xFF },
    /* 195 */ { 0xbf, 0x00, 0xbf, 0xFF },
    /* 196 */ { 0x9f, 0x00, 0x9f, 0xFF },
    /* 197 */ { 0x7f, 0x00, 0x7f, 0xFF },
    /* 198 */ { 0x5f, 0x00, 0x5f, 0xFF },
    /* 199 */ { 0x3f, 0x00, 0x3f, 0xFF },
    /* 200 */ { 0xe9, 0xe9, 0xf3, 0xFF },
    /* 201 */ { 0xc4, 0xc4, 0xe1, 0xFF },
    /* 202 */ { 0x9d, 0x9d, 0xce, 0xFF },
    /* 203 */ { 0x77, 0x77, 0xbb, 0xFF },
    /* 204 */ { 0x54, 0x54, 0xa7, 0xFF },
    /* 205 */ { 0x41, 0x41, 0x83, 0xFF },
    /* 206 */ { 0x2e, 0x2e, 0x5c, 0xFF },
    /* 207 */ { 0x1b, 0x1b, 0x34, 0xFF },
    /* 208 */ { 0xd5, 0xf1, 0xff, 0xFF },
    /* 209 */ { 0xbf, 0xeb, 0xff, 0xFF },
    /* 210 */ { 0xaa, 0xe3, 0xff, 0xFF },
    /* 211 */ { 0x95, 0xdd, 0xff, 0xFF },
    /* 212 */ { 0x80, 0xd6, 0xff, 0xFF },
    /* 213 */ { 0x6a, 0xcf, 0xff, 0xFF },
    /* 214 */ { 0x55, 0xc8, 0xff, 0xFF },
    /* 215 */ { 0x3f, 0xbf, 0xff, 0xFF },
    /* 216 */ { 0x37, 0x9d, 0xdf, 0xFF },
    /* 217 */ { 0x2f, 0x8f, 0xbf, 0xFF },
    /* 218 */ { 0x27, 0x77, 0x9f, 0xFF },
    /* 219 */ { 0x1f, 0x5f, 0x7f, 0xFF },
    /* 220 */ { 0x00, 0xbf, 0xbf, 0xFF },
    /* 221 */ { 0x00, 0x7f, 0x7f, 0xFF },
    /* 222 */ { 0x00, 0x5f, 0x5f, 0xFF },
    /* 223 */ { 0x00, 0x3f, 0x3f, 0xFF },
    /* 224 */ { 0xe7, 0xe7, 0xff, 0xFF },
    /* 225 */ { 0xc6, 0xc6, 0xff, 0xFF },
    /* 226 */ { 0xad, 0xad, 0xff, 0xFF },
    /* 227 */ { 0x8c, 0x8c, 0xff, 0xFF },
    /* 228 */ { 0x73, 0x73, 0xff, 0xFF },
    /* 229 */ { 0x52, 0x52, 0xff, 0xFF },
    /* 230 */ { 0x31, 0x31, 0xff, 0xFF },
    /* 231 */ { 0x18, 0x18, 0xff, 0xFF },
    /* 232 */ { 0x00, 0x00, 0xff, 0xFF },
    /* 233 */ { 0x00, 0x00, 0xe7, 0xFF },
    /* 234 */ { 0x00, 0x00, 0xce, 0xFF },
    /* 235 */ { 0x00, 0x00, 0xb5, 0xFF },
    /* 236 */ { 0x00, 0x00, 0x9c, 0xFF },
    /* 237 */ { 0x00, 0x00, 0x84, 0xFF },
    /* 238 */ { 0x00, 0x00, 0x6b, 0xFF },
    /* 239 */ { 0x00, 0x00, 0x52, 0xFF },
    /* 240 */ { 0x00, 0x00, 0x4f, 0xFF },
    /* 241 */ { 0x00, 0x00, 0x3f, 0xFF },
    /* 242 */ { 0x00, 0x00, 0x37, 0xFF },
    /* 243 */ { 0x00, 0x00, 0x27, 0xFF },
    /* 244 */ { 0x00, 0x00, 0x1f, 0xFF },
    /* 245 */ { 0x00, 0x00, 0x0f, 0xFF },
    /* 246 */ { 0x00, 0x00, 0x07, 0xFF },
    /* 247 */ { 0x00, 0xff, 0xff, 0xFF },
    /* 248 */ { 0xcf, 0x7f, 0xcf, 0xFF },
    /* 249 */ { 0xb7, 0x6f, 0xb7, 0xFF },
    /* 250 */ { 0x9f, 0x5f, 0x9f, 0xFF },
    /* 251 */ { 0x87, 0x4f, 0x87, 0xFF },
    /* 252 */ { 0x6f, 0x3f, 0x6f, 0xFF },
    /* 253 */ { 0x57, 0x2f, 0x57, 0xFF },
    /* 254 */ { 0x3f, 0x1f, 0x3f, 0xFF },
    /* 255 */ { 0x27, 0x0f, 0x27, 0xFF }
};

void ComputePixels(Uint8* pixels, int res, int w, int h) {
    int r = w / 4;
    if (h < w) {
        r = h / 4;
    }
    for (int x = ((res-1)/2); x < w; x += res) {
        for (int y = ((res-1)/2); y < h; y += res) {
            float c_x = (float)(x - w/2) / r, c_y = (float)(y - h/2) / r;
            float z_x = 0, z_y = 0;
            float m = 0;
            int n = 0;
            while (m < DIV_LIMIT && n < 256) {
                float w_x = z_x, w_y = z_y;
                z_x = c_x + w_x*w_x - w_y*w_y;
                z_y = c_y + 2*w_x*w_y;
                m = z_x*z_x + z_y*z_y;
                n++;
                SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "c = %.2f + %.2fi; z_%d = %.2f + %.2fi", c_x, c_y, n, z_x, z_y);
            }
            SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "c = %.2f + %.2fi; n = %d; m = %.1f", c_x, c_y, n, m);
            for (int i = -((res-1)/2); i <= ((res-1)/2); i++) {
                for (int j = -((res-1)/2); j <= ((res-1)/2); j++) {
                    int idx = (y+j)*w + x + i;
                    if (idx >= 0 && idx < w*h) {
                        pixels[idx] = 256 - n;
                    }
                }
            }
        }
    }
}

SDL_Texture* Draw(SDL_Renderer* renderer, int res, SDL_Rect rect) {
    int err = SUCCESS;
    SDL_Texture* texture = NULL;
    SDL_Surface* surface = NULL;
    SDL_Palette* palette = NULL;
    Uint8* pixels = (Uint8*)calloc(rect.w*rect.h, sizeof(Uint8));

    ComputePixels(pixels, res, rect.w, rect.h);

    surface = SDL_CreateRGBSurfaceWithFormatFrom(
        (void*)pixels,         // pixels
        rect.w,                // width
        rect.h,                // height
        8,                     // depth
        rect.w,                // pitch
        SDL_PIXELFORMAT_INDEX8 // format
    );
    if (surface == NULL) {
        free(pixels);
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to create SDL surface: %s", SDL_GetError());
        return texture;
    }

    err = SDL_SetPaletteColors(surface->format->palette, colors, 0, 1<<8);
    if (err != SUCCESS) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to set palette colors: %s", SDL_GetError());
    } else {
        texture = SDL_CreateTextureFromSurface(renderer, surface);
    }
    SDL_FreeSurface(surface);
    free(pixels);
    return texture;
}

int ClearRenderer(SDL_Renderer *renderer) {
    int err = SUCCESS;

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "Clearing SDL renderer...");
    err = SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
    if (err != SUCCESS) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to set draw color: %s", SDL_GetError());
        return err;
    }
    err = SDL_RenderClear(renderer);
    if (err != SUCCESS) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to clear SDL renderer: %s", SDL_GetError());
        return err;
    }
    SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "SDL renderer cleared.");

    return err;
}

int RenderFrame(SDL_Renderer *renderer, int res) {
    int err = SUCCESS;
    SDL_Rect viewport;
    SDL_Texture* texture = NULL;

    err = ClearRenderer(renderer);
    if (err != SUCCESS) {
        return err;
    }

    SDL_RenderGetViewport(renderer, &viewport);
    texture = Draw(renderer, res, viewport);
    if (texture == NULL) {
        return 1;
    }
    err = SDL_RenderCopy(renderer, texture, NULL, NULL);
    SDL_DestroyTexture(texture);
    if (err != SUCCESS) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to copy texture to renderer: %s", SDL_GetError());
        return err;
    }

    SDL_RenderPresent(renderer);

    TrackFrameRate();

    return err;
}

int Setup(SDL_Window **window, SDL_Renderer **renderer) {
    *window = NULL;
    *renderer = NULL;

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "Creating SDL window...");
    *window = SDL_CreateWindow("Mandelbrot",
        SDL_WINDOWPOS_CENTERED,  // x
        SDL_WINDOWPOS_CENTERED,  // y
        640,                     // width
        640,                     // height
        SDL_WINDOW_SHOWN         // flags
        //| SDL_WINDOW_BORDERLESS
        | SDL_WINDOW_RESIZABLE
    );
    if (*window == NULL) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to create SDL window: %s", SDL_GetError());
        return 1;
    }
    SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "SDL window created.");

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "Creating SDL renderer...");
    *renderer = SDL_CreateRenderer(
        *window,
        -1,                         // initialize the first driver matching flags
        SDL_RENDERER_ACCELERATED    // flags
        //| SDL_RENDERER_PRESENTVSYNC
    );
    if (*renderer == NULL) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to create SDL renderer: %s", SDL_GetError());
        return 1;
    }
    SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "SDL renderer created.");

    return SUCCESS;
}

int MainLoop(SDL_Renderer *renderer) {
    int err = SUCCESS;
    int res = 25;

    while (1) {
        SDL_Event event;

        if (res > 0) {
            err = RenderFrame(renderer, res);
            if (err != SUCCESS) {
                return err;
            }
            res -= 2;
        }

        while (SDL_PollEvent(&event)) {
            switch (event.type) {
            case SDL_QUIT:
                return err;
            case SDL_WINDOWEVENT:
                switch (event.window.event) {
                case SDL_WINDOWEVENT_RESIZED:
                case SDL_WINDOWEVENT_SIZE_CHANGED:
                    res = 25;
                    break;
                }
                break;
            }
        }
    }

    return err;
}

int main(int argc, const char* argv[])
{
    int err = SUCCESS;
    SDL_Window *window = NULL;
    SDL_Renderer *renderer = NULL;

    SDL_LogSetPriority(SDL_LOG_CATEGORY_APPLICATION, SDL_LOG_PRIORITY_INFO);

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "Initializing SDL...");
    if (SDL_Init(SDL_INIT_VIDEO) != SUCCESS) {
        SDL_LogError(SDL_LOG_CATEGORY_APPLICATION, "Unable to initialize SDL: %s", SDL_GetError());
        return 1;
    }
    SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "SDL initialized.");

    err = Setup(&window, &renderer);

    if (err == SUCCESS) {
        err = MainLoop(renderer);
    }

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION, "Destroying SDL resources...");
    if (renderer != NULL) {
        SDL_DestroyRenderer(renderer);
    }
    if (window != NULL) {
        SDL_DestroyWindow(window);
    }
    SDL_LogDebug(SDL_LOG_CATEGORY_APPLICATION, "SDL resources destroyed.");

    SDL_Quit();
    return err;
}
