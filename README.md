### Reflection Publisher-1

1. In the Observer pattern, a `Subscriber` is typically an interface or trait to allow flexibility in defining multiple concrete subscribers. However, in our case, using a single `struct` is sufficient since the system has a predefined and fixed subscriber behavior. There is no need to dynamically vary the behavior of different subscribers, so we don't benefit much from using a trait.

2. The `id` in `Program` and the `url` in `Subscriber` must be unique to ensure no duplicate entries. Using a `Vec` would require us to manually check for duplicates on each operation, which is inefficient. `DashMap` allows constant time lookup, insert, and delete operations, making it more suitable for this use case where quick uniqueness validation is needed.

3. Rust enforces thread safety at the compiler level, but we still need runtime guarantees when sharing state across threads. `DashMap` provides thread-safe access to a HashMap, which is necessary for our use case since `SUBSCRIBERS` is a global mutable state accessed concurrently. Although the Singleton pattern ensures a single instance, it does not provide concurrency safety by itself. Thus, we still need `DashMap` to guarantee safe concurrent access.

### Reflection Publisher-2

1. In the traditional MVC (Model-View-Controller) pattern, the Model is responsible for both data storage and business logic. However, as software grows more complex, separating responsibilities becomes crucial. By introducing `Service` and `Repository` layers, we adhere to the Single Responsibility Principle (SRP). The `Repository` focuses only on data access, while the `Service` handles business logic. This separation improves testability, maintainability, and scalability of our application.

2. If we only use the `Model`, all logic—including data manipulation and business rules—would be tightly coupled inside it. This creates high complexity and low cohesion. For example, managing `Program`, `Subscriber`, and `Notification` interactions in a single struct would make the model bloated and hard to manage. Adding or changing functionality in one model would risk introducing bugs into unrelated parts. Separating logic reduces interdependency and simplifies the codebase.

3. Postman is an extremely helpful tool for testing HTTP endpoints. It allows us to simulate client requests without writing a frontend. In this project, we used Postman to test our `subscribe` and `unsubscribe` endpoints by sending raw JSON requests and inspecting the responses. Postman’s ability to save collections, write tests, and chain requests is especially useful for teamwork and automation. We plan to use it in our Group Project for testing auth flows and complex multi-step API sequences.

### Reflection Publisher-3

1. In this tutorial, we use the **Push model** of the Observer Pattern, where the **publisher actively sends** data (notifications) to subscribers. When a product is created, promoted, or deleted, the system calls `NotificationService::notify`, which immediately sends HTTP POST requests to each subscriber’s URL.

2. If we used the **Pull model** instead, subscribers would be responsible for periodically requesting updates from the publisher.  
   - **Advantages of Pull**:
     - Subscribers can control how often they fetch updates (more flexible).
     - Less load on publisher since it doesn’t need to notify all.
   - **Disadvantages of Pull**:
     - Delayed notification (not real-time).
     - Increased complexity on the subscriber side.
     - Redundant or unnecessary polling.

   In this case, the Push model is better because we need real-time delivery of product notifications to subscribers without them having to request updates manually.

3. Without **multi-threading**, the notification process would become **blocking and sequential**. This means:
   - Each subscriber's HTTP request would have to wait for the previous one to finish.
   - If one subscriber is slow or unresponsive, it will **delay notifications** for others.
   - Overall, the system becomes inefficient and may even appear frozen during high-load scenarios.

   By using multi-threading (with `thread::spawn`), we send notifications **concurrently**, ensuring fast and non-blocking execution of all updates.
