#include <cstdint>

extern "C" {
    // Placeholder C++ simulation bridge functions called via FFI
    void* sim_init(uint64_t base_addr) {
        return nullptr; // Stub for actual Verilator top-level object pointer
    }

    void sim_step(void* handle, uint64_t cycles) {
        // Drive clock cycles
    }

    uint32_t sim_read_mem(void* handle, uint32_t addr) {
        return 0x00000000;
    }

    void sim_write_mem(void* handle, uint32_t addr, uint32_t data) {
        // Handle write transaction
    }

    void sim_destroy(void* handle) {
        // Clean up simulation context
    }
}
