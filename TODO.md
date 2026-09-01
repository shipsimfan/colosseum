# ToDo
 0. Separate "doubled" buffers into staging + device local buffers with a copy fence protecting the staging buffers
   1. Add `DeviceDataBuffer` to `FrameData` for camera, renderables, light metadata, directional lights, point lights, and spot lights
   2. Add updating of descriptor sets when data buffers resize
   3. Add copy commands to command buffer for copying staging buffers to device local buffers
   4. Add wait for copy fence to start of `ecs.execute_rendering_systems`
 1. Fix resize issue
 2. Add procedural skybox
 3. Add shadow mapping
   1. Add shadow map memory container
   2. Spot light shadows
   3. Directional light shadows
   4. Point light shadows
 4. Combine all GPU transfer staging buffers into one 
 5. Add texture loading
 6. Add mesh loading
 7. Add PBR material system
 8. Add transient texture aliasing system
 9. Add color grading to tone map node
   1. Add CPU LUT type w/ SlotMap
   2. Add render side LUT (dedicated allocations)
   3. Add LUTs to transfer queue
   4. Add per-frame descriptor set for color grading LUT
   5. Add descriptor set to pipeline
   6. Add updating of descriptor set when LUT changes (in tone map node)
   7. Add binding of descriptor set to tone map node
   8. Add sampling of LUT in tone map node fragment shader
   9. Add reading .cube files
 10. Add vignette
 11. Add bloom
 12. Add lens flare
 13. Add deferred rendering
 14. Add physics system
 15. Add collision system
 16. Add audio system
 17. Add mouse support
 18. Add UI system
 19. Add gamepad support
 20. Add other input support
 21. Add auto-exposure to tone mapping
 22. Add input translation system
 23. Add multiple keyboard support
 24. Add multiple mouse support
 25. Add resource and dependency support for ECS systems
 26. Add TAA
 27. Add job system
 28. Parallelize frame graph execution
 29. Parallelize ECS systems
 30. Pipeline update and render jobs
 31. Add localization system
 32. Add VR support