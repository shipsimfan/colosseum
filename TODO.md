# ToDo
 1. Rewrite ECS
   1. Update `Commands` system parameter for entities
     1. Add entity
     2. Remove entity
     3. Add component
     4. Remove component
   2. Implement `Query` system parameter
     1. Register existing archetypes with new systems
     2. Register new archetypes with existing systems
     3. Single component
     4. Multiple components
     5. Optional components
     6. Filters
       1. `With` filter
       2. `Without` filter
   3. Update remaining engine to new ECS
     1. Add unlit rendering
     2. Add procedural skybox
     3. Add lit rendering
     4. Add shadow mapping
 2. Add mesh loading
 3. Add raycasting
 4. Add texture loading
 5. Add PBR material system
 6. Add physics system
 7. Add collision system
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
 9. Add audio system
 10. Add mouse support
 11. Add UI system
 12. Add gamepad support
 13. Add Poisson Disk PCF with kernel size controlled by shadow quality
 14. Add depth bias settings for shadow mapping
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
 32. Parallelize individual systems (e.g. rendering, physics)
 33. Parallelize systems within a phase
 34. Parallelize phases
 35. Add localization system
 36. Add VR support
 37. Add TAA