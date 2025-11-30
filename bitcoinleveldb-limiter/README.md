# bitcoinleveldb-limiter

`bitcoinleveldb-limiter` is a Rust crate designed to control and limit resource usage, specifically targeting the scarcity of file descriptors and mmap utilization. By imposing constraints, it effectively mitigates potential exhaustion issues in applications that handle large datasets, ensuring that kernel performance remains optimal.

## Features

- **Resource Limiting**: Adjust the number of resources available, preventing depletion of file descriptors and virtual memory.
- **Concurrency Safety**: Utilize atomic operations to manage state changes, supporting concurrent environments.
- **Dynamic Adjustments**: Capable of modifying the allowed resources in real-time based on need.

## Usage

To use the `Limiter`, instantiate it with a maximum number of permissible acquisitions. The API supports acquiring and releasing resources, ensuring that usage stays within defined bounds.

```rust
use bitcoinleveldb_limiter::Limiter;

fn main() {
    let limiter = Limiter::new(10);

    if limiter.acquire() {
        println!("Resource acquired");
        // Use resource here
        limiter.release();
    } else {
        println!("Resource not available");
    }
}
```

### Important Considerations
- Attempts to acquire resources decrement the available count if successful.
- Releasing increments the count, supporting re-use and recycling of permits.
- Debug assertions aid in identifying inconsistencies in acquire/release operations.

## Contributing
As a foundational utility, your contributions are valuable. Please file issues or submit pull requests on [GitHub](https://github.com/klebs6/bitcoin-rs).

## License
This project is licensed under the MIT License.