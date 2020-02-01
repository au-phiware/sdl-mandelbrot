#include <stdio.h>
#include <SDL2/SDL.h>

#define SUCCESS 0

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

int RenderFrame(SDL_Renderer *renderer) {
    int err = SUCCESS;

    err = ClearRenderer(renderer);
    if (err != SUCCESS) {
        return err;
    }

    SDL_RenderPresent(renderer);

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
        | SDL_WINDOW_BORDERLESS
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

    while (1) {
        SDL_Event event;

        err = RenderFrame(renderer);

        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                return err;
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
