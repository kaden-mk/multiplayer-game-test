-- LEMON SIGNAL PORTED OVER

--!native
--!nocheck
--!nolint

export type Connection = {
    connected: boolean,

    disconnect: (self: Connection) -> (),
    reconnect: (self: Connection) -> ()
}

export type Signal<T...> = {
   	connect: (self: Signal<T...>, fn: (T...) -> ()) -> Connection,
	once: (self: Signal<T...>, fn: (T...) -> ()) -> Connection,
	wait: (self: Signal<T...>) -> T...,
	fire: (self: Signal<T...>, T...) -> (),
	disconnect_all: (self: Signal<T...>) -> (),
	destroy: (self: Signal<T...>) -> (),
}

const free_threads: { thread } = {}

local function run_callback(callback, thread, ...)
    callback(...)
    table.insert(free_threads, thread)
end

local function yielder()
    while true do
        run_callback(coroutine.yield())
    end
end

const Connection = {}
Connection.__index = Connection

const function disconnect(self: Connection)
    if not self.connected then
		return
	end
	self.connected = false

	const next = self._next
	const prev = self._prev

	if next then
		next._prev = prev
	end
	if prev then
		prev._next = next
	end

	const signal = self._signal
	if signal._head == self then
		signal._head = next
	end
end

const function reconnect(self: Connection)
    if self.connected then
		return
	end
	self.connected = true

	const signal = self._signal
	const head = signal._head
	if head then
		head._prev = self
	end
	signal._head = self

	self._next = head
	self._prev = false
end

Connection.disconnect = disconnect
Connection.reconnect = reconnect

--\\ Signal //--
const Signal = {}
Signal.__index = Signal

const function connect<T...>(self: Signal<T...>, fn: (T...) -> ()): Connection
    const head = self._head
	const cn = setmetatable({
		connected = true,
		_signal = self,
		_fn = fn,
		_next = head,
		_prev = false,
	}, Connection)

	if head then
		head._prev = cn
	end
	self._head = cn

	return cn
end

const function once<T...>(self: Signal<T...>, fn: (T...) -> ()): Connection
	local cn
	cn = connect(self, function(...)
		disconnect(cn)
		fn(...)
	end)

	return cn
end

const wait = if task
	then function<T...>(self: Signal<T...>): ...any
		local thread = coroutine.running()
		local cn
		cn = connect(self, function(...)
			disconnect(cn)
			if coroutine.status(thread) == "suspended" then
				task.spawn(thread, ...)
			end
		end)
		return coroutine.yield()
	end
	else function<T...>(self: Signal<T...>): ...any
		local thread = coroutine.running()
		local cn
		cn = connect(self, function(...)
			disconnect(cn)
			local passed, message = coroutine.resume(thread, ...)
			if not passed then
				error(message, 0)
			end
		end)
		return coroutine.yield()
	end

const fire = if task
	then function<T...>(self: Signal<T...>, ...: any)
		local cn = self._head
		while cn do
			local thread
			if #free_threads > 0 then
				thread = free_threads[#free_threads]
				free_threads[#free_threads] = nil
			else
				thread = coroutine.create(yielder)
				coroutine.resume(thread)
			end

			task.spawn(thread, cn._fn, thread, ...)

			cn = cn._next
		end
	end
	else function<T...>(self: Signal<T...>, ...: any)
		local cn = self._head
		while cn do
			local thread
			if #free_threads > 0 then
				thread = free_threads[#free_threads]
				free_threads[#free_threads] = nil
			else
				thread = coroutine.create(yielder)
				coroutine.resume(thread)
			end

			local passed, message = coroutine.resume(thread, cn._fn, thread, ...)
			if not passed then
				print(string.format("%s\nstacktrace:\n%s", message, debug.traceback()))
			end

			cn = cn._next
		end
	end

local function disconnect_all<T...>(self: Signal<T...>)
	local cn = self._head
	while cn do
		disconnect(cn)
		cn = cn._next
	end
end

local function destroy<T...>(self: Signal<T...>)
	disconnect_all(self)
end

--\\ Constructors
function Signal.new<T...>(): Signal<T...>
	return setmetatable({ _head = false }, Signal)
end

Signal.connect = connect
Signal.once = once
Signal.wait = wait
Signal.fire = fire
Signal.disconnect_all = disconnect_all
Signal.destroy = destroy

return { new = Signal.new }
