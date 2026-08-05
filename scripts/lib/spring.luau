const Spring = {}
const SpringModule = {}
SpringModule.__index = Spring

type getter_function<T> = (self: Spring<T>) -> T

export type Spring<T> = {
    frequency: number,
    damping: number,

    position: T,
    goal: T,
    velocity: T,

    update: (self: Spring<T>, dt: number) -> T,
    impulse: (self: Spring<T>, v: T) -> (),
    set_goal: (self: Spring<T>, goal: T) -> (),
    set: (self: Spring<T>, new_position: T) -> (),

    get_position: getter_function<T>,
    get_velocity: getter_function<T>
}

function SpringModule.new<T>(initial: T, config: { frequency: number?, damping: number? }?): Spring<T>
    const c = config or {}

    const self = setmetatable({
        frequency = c.frequency or 8,
        damping = c.damping or 0.6,

        position = initial,
        goal = initial,
        velocity = (initial :: any) * 0
    }, SpringModule)

    return self
end

const SUB_STEPS = 4
function Spring.update<T>(self: Spring<T>, dt: number): T
    const safe_dt = math.min(dt, 0.05)
    
    const sub_dt = safe_dt / SUB_STEPS

    const frequency = self.frequency
    const damping = self.damping
    const f2 = frequency * frequency
    const damping_term = 2 * damping * frequency

    local position = self.position :: any
    local velocity = self.velocity :: any
    const goal = self.goal :: any

    for _ = 1, SUB_STEPS do
        const accel = -f2 * (position - goal) - damping_term * velocity
        velocity += accel * sub_dt
        position += velocity * sub_dt
    end

    self.velocity = velocity
    self.position = position
    return position
end

function Spring.impulse<T>(self: Spring<T>, v: T): ()
    self.velocity = (self.velocity :: any) + v
end

function Spring.set_goal<T>(self: Spring<T>, goal: T): ()
    self.goal = goal
end

function Spring.set<T>(self: Spring<T>, new_position: T): ()
    self.velocity = (self.velocity :: any) * 0
    self.position = new_position
end

function Spring.get_position<T>(self: Spring<T>): T
    return self.position
end

function Spring.get_velocity<T>(self: Spring<T>): T
    return self.velocity
end

return SpringModule
