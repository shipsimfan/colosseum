# ToDo
 1. Add HDR
   1. Add adjustable gamma push constant
     1. Add gamma to settings
     2. Add gamma to render data
     3. Set gamma during update
     4. Push gamma to shader
     5. Use gamma in fragment shader
   2. Add dithering using interleaved gradient noise (after gamma correction, in same shader)
   3. Add tunable sharpening to gamma correction node (before gamma correction itself, AMD CAS)
   4. Add ToneMapNode w/ no change fragment shader (before render scale, after main render)
   5. Add HDR without tone mapping
     1. Add HDR render target
     2. Adjust pipeline formats to R16G16B16A16SFloat, except for the tone mapping node's itself
   6. Add Narkawicz approximation for ACES filmic
   7. Add adjustable exposure push constant
     1. Add exposure to render data
     2. Set exposure during update
     3. Push exposure to shader
     4. Use exposure in fragment shader
 2. Add color grading to tone map node
   1. Add CPU LUT type w/ SlotMap
   2. Add render side LUT (dedicated allocations)
   3. Add LUTs to transfer queue
   4. Add per-frame descriptor set for color grading LUT
   5. Add descriptor set to pipeline
   6. Add updating of descriptor set when LUT changes (in tone map node)
   7. Add binding of descriptor set to tone map node
   8. Add sampling of LUT in tone map node fragment shader
 3. Add dithering (after gamma correction, in same shader)
 4. Add FXAA (after render scale node, before gamma correction)
 5. Add lighting system
   1. Add lit opaque materials
   2. Directional lights
   3. Point lights
   4. Spot lights
 6. Add shadow mapping
   1. Directional light shadows
   2. Point light shadows
   3. Spot light shadows
 7. Add mesh loading
 8. Add texture loading
 9. Add PBR material system
 10. Add transient texture aliasing system
 11. Add vignette
 12. Add bloom
 13. Add lens flare
 14. Add deferred rendering
 15. Add physics system
 16. Add collision system
 17. Add audio system
 18. Add mouse support
 19. Add UI system
 20. Add gamepad support
 21. Add other input support
 22. Add auto-exposure to tone mapping
 23. Add input translation system
 24. Add multiple keyboard support
 25. Add multiple mouse support
 26. Add resource and dependency support for ECS systems
 27. Add TAA
 28. Add job system
 29. Parallelize frame graph execution
 30. Parallelize ECS systems
 31. Pipeline update and render jobs
 32. Add localization system
 33. Add VR support