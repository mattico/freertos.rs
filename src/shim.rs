use base::*;


#[cfg(target_os="none")]
extern {
	pub fn freertos_rs_sizeof(_type: u8) -> u8;

	pub fn freertos_rs_vTaskDelayUntil(pxPreviousWakeTime: *mut FreeRtosTickType, xTimeIncrement: FreeRtosTickType);
	pub fn freertos_rs_vTaskDelay(xTicksToDelay: FreeRtosTickType);
	pub fn freertos_rs_get_portTICK_PERIOD_MS() -> FreeRtosTickType;

	pub fn freertos_rs_xTaskGetTickCount() -> FreeRtosTickType;

	pub fn freertos_rs_create_recursive_mutex() -> FreeRtosQueueHandle;
	pub fn freertos_rs_create_mutex() -> FreeRtosQueueHandle;	
	
	pub fn freertos_rs_take_recursive_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType;
	pub fn freertos_rs_take_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType;
	pub fn freertos_rs_give_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType;	
	pub fn freertos_rs_give_recursive_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType;

	pub fn freertos_rs_delete_semaphore(mutex: FreeRtosQueueHandle);

	pub fn freertos_rs_create_binary_semaphore() -> FreeRtosQueueHandle;
	pub fn freertos_rs_create_counting_semaphore(max: FreeRtosUBaseType, initial: FreeRtosUBaseType) -> FreeRtosQueueHandle;

	//pub fn freertos_rs_take_semaphore_isr(semaphore: FreeRtosQueueHandle, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType;
	//pub fn freertos_rs_give_semaphore_isr(semaphore: FreeRtosQueueHandle, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType;
	
	//pub fn freertos_rs_get_semaphore_count(mutex: *const c_void) -> u32;

	pub fn freertos_rs_queue_create(length: FreeRtosUBaseType, item_size: FreeRtosUBaseType) -> FreeRtosQueueHandle;
	pub fn freertos_rs_queue_delete(queue: FreeRtosQueueHandle);
	pub fn freertos_rs_queue_send(queue: FreeRtosQueueHandle, item: FreeRtosVoidPtr, max_wait: FreeRtosTickType) -> FreeRtosUBaseType;
	pub fn freertos_rs_queue_receive(queue: FreeRtosQueueHandle, item: FreeRtosMutVoidPtr, max_wait: FreeRtosTickType) -> FreeRtosUBaseType;

	pub fn freertos_rs_queue_send_isr(queue: FreeRtosQueueHandle, item: FreeRtosVoidPtr, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosUBaseType;
	pub fn freertos_rs_isr_yield();

	pub fn freertos_rs_task_notify_take(clear_count: u8, wait: FreeRtosTickType) -> u32;
	pub fn freertos_rs_task_notify_wait(ulBitsToClearOnEntry: u32, ulBitsToClearOnExit: u32, pulNotificationValue: *mut u32, xTicksToWait: FreeRtosTickType) -> FreeRtosBaseType;

	pub fn freertos_rs_task_notify(task: FreeRtosTaskHandle, value: u32, action: u8) -> FreeRtosBaseType;
	pub fn freertos_rs_task_notify_isr(task: FreeRtosTaskHandle, value: u32, action: u8, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType;

	pub fn freertos_rs_spawn_task(f: extern fn(FreeRtosMutVoidPtr) -> FreeRtosMutVoidPtr, value: FreeRtosMutVoidPtr, name: FreeRtosCharPtr, name_len: u8, stack_size: u16, priority: FreeRtosUBaseType, task_handle: FreeRtosMutTaskHandle) -> FreeRtosUBaseType;
	pub fn freertos_rs_delete_task(task: FreeRtosTaskHandle);
	pub fn freertos_rs_task_get_name(task: FreeRtosTaskHandle) -> FreeRtosCharPtr;

	pub fn freertos_rs_get_current_task() -> FreeRtosTaskHandle;

	pub fn freertos_rs_max_wait() -> FreeRtosTickType;
}

// mocks for testing
#[cfg(not(target_os="none"))]
pub mod freertos_rs_mocked {
	use base::*;

	pub fn freertos_rs_sizeof(_type: u8) -> u8 { 0 }

	pub fn freertos_rs_vTaskDelayUntil(pxPreviousWakeTime: *mut FreeRtosTickType, xTimeIncrement: FreeRtosTickType) { }
	pub fn freertos_rs_vTaskDelay(xTicksToDelay: FreeRtosTickType) { }
	pub fn freertos_rs_get_portTICK_PERIOD_MS() -> FreeRtosTickType { 1 }

	pub fn freertos_rs_xTaskGetTickCount() -> FreeRtosTickType { 1 }

	pub fn freertos_rs_create_recursive_mutex() -> FreeRtosQueueHandle { 1 as _ }
	pub fn freertos_rs_create_mutex() -> FreeRtosQueueHandle { 1 as _ }
	
	pub fn freertos_rs_take_recursive_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType { 0 }
	pub fn freertos_rs_take_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType { 0 }
	pub fn freertos_rs_give_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType { 0 }
	pub fn freertos_rs_give_recursive_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType { 0 }

	pub fn freertos_rs_delete_semaphore(mutex: FreeRtosQueueHandle) { }

	pub fn freertos_rs_create_binary_semaphore() -> FreeRtosQueueHandle { 1 as _ }
	pub fn freertos_rs_create_counting_semaphore(max: FreeRtosUBaseType, initial: FreeRtosUBaseType) -> FreeRtosQueueHandle { 1 as _ }

	//pub fn freertos_rs_take_semaphore_isr(semaphore: FreeRtosQueueHandle, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType;
	//pub fn freertos_rs_give_semaphore_isr(semaphore: FreeRtosQueueHandle, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType;
	
	//pub fn freertos_rs_get_semaphore_count(mutex: *const c_void) -> u32;

	pub fn freertos_rs_queue_create(length: FreeRtosUBaseType, item_size: FreeRtosUBaseType) -> FreeRtosQueueHandle { 1 as _ }
	pub fn freertos_rs_queue_delete(queue: FreeRtosQueueHandle) { }
	pub fn freertos_rs_queue_send(queue: FreeRtosQueueHandle, item: FreeRtosVoidPtr, max_wait: FreeRtosTickType) -> FreeRtosUBaseType { 0 }
	pub fn freertos_rs_queue_receive(queue: FreeRtosQueueHandle, item: FreeRtosMutVoidPtr, max_wait: FreeRtosTickType) -> FreeRtosUBaseType { 0 }

	pub fn freertos_rs_queue_send_isr(queue: FreeRtosQueueHandle, item: FreeRtosVoidPtr, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosUBaseType { 0 }
	pub fn freertos_rs_isr_yield() { }

	pub fn freertos_rs_task_notify_take(clear_count: u8, wait: FreeRtosTickType) -> u32 { 0 }
	pub fn freertos_rs_task_notify_wait(ulBitsToClearOnEntry: u32, ulBitsToClearOnExit: u32, pulNotificationValue: *mut u32, xTicksToWait: FreeRtosTickType) -> FreeRtosBaseType { 0 }

	pub fn freertos_rs_task_notify(task: FreeRtosTaskHandle, value: u32, action: u8) -> FreeRtosBaseType { 0 }
	pub fn freertos_rs_task_notify_isr(task: FreeRtosTaskHandle, value: u32, action: u8, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr) -> FreeRtosBaseType { 0 }

	pub fn freertos_rs_spawn_task(f: extern fn(FreeRtosMutVoidPtr) -> FreeRtosMutVoidPtr, value: FreeRtosMutVoidPtr, name: FreeRtosCharPtr, name_len: u8, stack_size: u16, priority: FreeRtosUBaseType, task_handle: FreeRtosMutTaskHandle) -> FreeRtosUBaseType { 0 }
	pub fn freertos_rs_delete_task(task: FreeRtosTaskHandle) { }
	pub fn freertos_rs_task_get_name(task: FreeRtosTaskHandle) -> FreeRtosCharPtr { 0 as _ }

	pub fn freertos_rs_get_current_task() -> FreeRtosTaskHandle { 1 as _ }

	pub fn freertos_rs_max_wait() -> FreeRtosTickType { 1000 }
}

#[cfg(not(target_os="none"))]
pub use shim::freertos_rs_mocked::*;