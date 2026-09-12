# ToDo
 1. Add shadow mapping
   1. Directional light shadows
     - Add Vulkan 1.2 Features to alexandria (+ add binding count limits to graphics info example)
     - Generalize shadow map node for different light types
     - Use shadow map result in lit shader
     - Resolve shimmering
     - Add cascading shadow maps (using layering + instance rendering)
   2. Point light shadows
   3. Add soft-shadows (using hardware PCF)
 2. Combine all GPU transfer staging buffers into one 
 3. Continue adding names to all objects
 4. Add debug labels to all render passes
 5. Add mesh loading
 6. Add texture loading
 7. Add PBR material system
 8. Add color grading to tone map node
   1. Add CPU LUT type w/ SlotMap
   2. Add render side LUT (dedicated allocations)
   3. Add LUTs to transfer queue
   4. Add per-frame descriptor set for color grading LUT
   5. Add descriptor set to pipeline
   6. Add updating of descriptor set when LUT changes (in tone map node)
   7. Add binding of descriptor set to tone map node
   8. Add sampling of LUT in tone map node fragment shader
   9. Add reading .cube files
 9. Add physics system
 10. Add collision system
 11. Add audio system
 12. Add mouse support
 13. Add UI system
 14. Add gamepad support
 15. Add camera culling
 16. Add shadow map culling
 17. Add light culling
 18. Add transient texture aliasing system
 19. Add vignette
 20. Add bloom
 21. Add lens flare
 22. Add deferred rendering
 23. Add auto-exposure to tone mapping
 24. Add other input support
 25. Add multiple keyboard support
 26. Add multiple mouse support
 27. Add input translation system
 28. Add custom shadow map shaders to materials
 29. Add transparency
 30. Add resource and dependency support for ECS systems
 31. Add job system
 32. Parallelize frame graph execution
 33. Parallelize ECS systems
 34. Pipeline update and render jobs
 35. Add localization system
 36. Add VR support
 37. Add TAA