const update_calls = {} :: { [number]: { read callback: (dt: number) -> (), read priority: number } }
const draw_calls = {} :: { [number]: { read callback: () -> (), read priority: number }  }

const Scheduler = {}

function Scheduler.add_on_update(callback: (dt: number) -> (), priority: number)
    table.insert(update_calls, {
        callback = callback,
        priority = priority
    })

    table.sort(update_calls, function(a: { read priority: number }, b)
        return a.priority > b.priority
    end)
end

function Scheduler.add_on_draw(callback: () -> (), priority: number)
    table.insert(draw_calls, {
        callback = callback,
        priority = priority
    })

    table.sort(draw_calls, function(a: { read priority: number }, b)
        return a.priority > b.priority
    end)
end

function Scheduler.update(dt: number)
    for _, call in update_calls do
        call.callback(dt)
    end
end

function Scheduler.draw()
    for _, call in draw_calls do
        call.callback()
    end
end

return Scheduler
