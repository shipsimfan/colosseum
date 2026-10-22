# ToDo
 1. Rewrite ECS
   1. Implement frame graph compile
     1. Add anti-aliasing
     2. Add render scale node
     3. Add procedural skybox (w/ ability to re-compile frame-graph)
   2. Add mesh transferring
   3. Add delta time
   4. Add ability to close window
   5. Add window state saving to settings
   6. Add scene transition
     1. Add permanent resources
     2. Implement clearing and compacting
     3. Add resource for next scene
     4. Add checking scene transition at start of update
     5. Add `SceneTransition` phase
   7. Add archetype management
     1. `add_entity`
     2. `remove_entity`
     3. `add_component`
     4. `try_get`, `get`, `try_get_mut`, `get_mut`
     5. `try_remove_component`, `remove_component`
     6. `add` helper
   8. Implement `Query` system parameter
     1. Single component
     2. Multiple components
     3. Optional components
     4. Filters
       1. `With` filter
       2. `Without` filter
   9. Update `Commands` system parameter for entities
     1. Add entity
     2. Remove entity
     3. Add component
     4. Remove component
   10. Update remaining engine to new ECS
     1. Add unlit rendering
     2. Add lit rendering
     3. Add shadow mapping
 2. Add names to shader modules
 3. Add mesh loading
 4. Add raycasting
 5. Add texture loading
 6. Add PBR material system
 7. Add physics system
 8. Add collision system
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
 10. Add audio system
 11. Add mouse support
 12. Add UI system
 13. Add gamepad support
 14. Add Poisson Disk PCF with kernel size controlled by shadow quality
 15. Add depth bias settings for shadow mapping
 16. Add camera culling
 17. Add shadow map culling
 18. Add light culling
 19. Add transient texture aliasing system
 20. Add vignette
 21. Add bloom
 22. Add lens flare
 23. Add deferred rendering
 24. Add auto-exposure to tone mapping
 25. Add other input support
 26. Add multiple keyboard support
 27. Add multiple mouse support
 28. Add input translation system
 29. Add custom shadow map shaders to materials
 30. Add transparency
 31. Add resource and dependency support for ECS systems
 32. Add job system
 33. Parallelize individual systems (e.g. rendering, physics)
 34. Parallelize systems within a phase
 35. Parallelize phases
 36. Add localization system
 37. Add VR support
 38. Add TAA