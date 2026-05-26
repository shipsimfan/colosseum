# ToDo
 1. Finish settings macro
   1. Implement load function
   2. Implement save function
   3. Implement getter functions
   4. Implement setting functions
 2. Add main WSI thread
   1. Create Alexandria context (this may have to happen before thread manager creation)
   2. Create window
   3. Create Vulkan instance
   4. Create Vulkan surface
   5. Add event loop until quit
   6. Add quit event pushing to thread manager kill
 3. Add pacer thread
   1. Take Vulkan instance and surface as input
   2. Create Vulkan device
   3. Create command pool 
   4. Create swapchain + inflight frame data
   5. Basic render + present loop
   6. Swapchain resize signalled from WSI thread
 4. Add job system
 5. Add simple render job
 6. Add simple update job
 7. Add frame graph system
 8. Add input system
 9. Add ECS system
 10. Add unlit mesh rendering
 11. Add mesh primitives
 12. Add custom shader support
 13. Add post processing system
 14. Add render scale
 15. Add mesh loading
 16. Add texture loading
 17. Add material system
 18. Add lighting system
 19. Add shadow mapping
 20. Add deferred rendering
 21. Add anti-aliasing