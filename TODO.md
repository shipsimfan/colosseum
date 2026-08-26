# ToDo
 1. Combine per frame data in the graphics device into a single struct
 2. Add HDR
   1. Add linear, no anistropy sampler to fixed render objects
   2. Add sampled usage
   3. Add ToneMapNode w/ no change fragment shader (swapchain format output)
   4. Add constant gamma correction
   5. Add adjustable gamma push constant
     1. Add gamma to settings
     2. Add gamma to render data
     3. Set gamma during update
     4. Push gamma to shader
     5. Use gamma in fragment shader
   6. Add HDR without tone mapping
     1. Add HDR render target
     2. Adjust pipeline formats to R16G16B16A16SFloat, except for the tone mapping node's itself
   7. Add Narkawicz approximation for ACES filmic
   8. Add adjustable exposure push constant
     1. Add exposure to render data
     2. Set exposure during update
     3. Push exposure to shader
     4. Use exposure in fragment shader
 3. Add color grading to tone map node
   1. Add CPU LUT type w/ SlotMap
   2. Add render side LUT (dedicated allocations)
   3. Add LUTs to transfer queue
   4. Add per-frame descriptor set for color grading LUT
   5. Add descriptor set to pipeline
   6. Add updating of descriptor set when LUT changes (in tone map node)
   7. Add binding of descriptor set to tone map node
   8. Add sampling of LUT in tone map node fragment shader
 4. Add FXAA
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
 11. Add post processing system
 12. Add bloom
 13. Add deferred rendering
 14. Add physics system
 15. Add collision system
 16. Add audio system
 17. Add mouse support
 18. Add UI system
 19. Add gamepad support
 20. Add other input support
 21. Add input translation system
 22. Add multiple keyboard support
 23. Add multiple mouse support
 24. Add resource and dependency support for ECS systems
 25. Add support for sharing graphics and transfer queues
 26. Add TAA
 27. Add job system
 28. Parallelize frame graph execution
 29. Parallelize ECS systems
 30. Pipeline update and render jobs
 31. Add localization system
 32. Add VR support
   1. Wheel
   2. Pedals
   3. Joystick
   4. Throttle
   5. VR controllers
 33. Add input translation system
 34. Add multiple keyboard support
 35. Add multiple mouse support
 36. Add resource and dependency support for ECS systems
 37. Add support for sharing graphics and transfer queues
 38. Add TAA
 39. Add job system
 40. Parallelize frame graph execution
 41. Parallelize ECS systems
 42. Pipeline update and render jobs
 43. Add localization system
 44. Add VR support