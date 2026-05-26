# ToDo
 1. Add main WSI thread
   1. Create Alexandria context (this may have to happen before thread manager creation)
   2. Create window
   3. Create Vulkan instance
   4. Create Vulkan surface
   5. Add event loop until quit
   6. Add quit event pushing to thread manager kill
 2. Add pacer thread
   1. Take Vulkan instance and surface as input
   2. Create Vulkan device
   3. Create command pool 
   4. Create swapchain + inflight frame data
   5. Basic render + present loop
   6. Swapchain resize signalled from WSI thread
 3. Add job system
 4. Add simple render job
 5. Add simple update job
 6. Add frame graph system
 7. Add input system
 8. Add ECS system
 9. Add unlit mesh rendering
 10. Add mesh primitives
 11. Add custom shader support
 12. Add post processing system
 13. Add render scale
 14. Add mesh loading
 15. Add texture loading
 16. Add material system
 17. Add lighting system
 18. Add shadow mapping
 19. Add deferred rendering
 20. Add anti-aliasing