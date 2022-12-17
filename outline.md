


## Basic Rendering

- Disclaimer
	- One of many ways etc etc
	- Purpose of writing
		- Lack of tutorials using modern opengl, debug callbacks
		- Lack of examples for simple rendering

- Window creation, context, function loading
	- SDL2?
	- Winit nonsense

- Debug callbacks

- Minimum setup to render
	- Shader compilation
		- TODO: Investigate shader pipelines
	- VAO
	- Draw square without vertex arrays

- Buffers
	- Projection
	- Basic Mesh building
		- Discuss cost of uploading every frame vs simplicity/flexibility
		- Demonstrate w/ benchmark

- Srgb correctness

- Texture loading
	- Update mesh building w/ uvs
	- Introduce sprites + establish pixel density concept

- Sprites in 3D
	- Different kinds of transforms
		- Billboards, planes
		- Scale factors
	- UI pass and layout

- TODO: Explore interop with aseprite?
	- Loading sprites from json


## Intermediate Rendering

- Post effects
	- Crunchy graphics
	- Fog, point lighting

- Drop shadows

- Dithered alpha

- Particles

- Alternative sprite rendering methods?
	- Sprite data in SSBO, programmable vertex pulling
	- Point sprites

	
## Future Topics??

- Input handling
- Basic audio
- Basic game states?
	- Screens
	- Transitions
	- Structuring different sections of a game
- Player movement
- Basic interaction 
- Debug UI
	- Simple tools with serde