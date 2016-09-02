var spawn = require('child_process').spawn

var outstanding = {}
var rust = spawn('target/debug/aws_lambda')

var buffer = ''

rust.stdout.on('data', (data) => {
    var obj = JSON.parse(data) // TODO: This is fragile
    outstanding[obj.invokeid](obj.response)
})

rust.stderr.pipe(process.stderr)

exports.handler = function(event, context) {
    var done = context.done
    if(typeof context == 'function')
        context = stupidHack(context)

    outstanding[context.invokeid] = function(response) {
        // TODO: Error handling... (error result, binary crashes, etc)
        done(null, response)
    }

    rust.stdin.write(JSON.stringify({event,context})+'\n')
}

// `local-lambda` decided that it was ok to make the context object a function
// this prevents the context variable from being stringifyied. This changes the
// context variable back into a regular object.
function stupidHack(fnctx) {
    var ctx = {}
    for(var key in fnctx)
        ctx[key] = fnctx[key]
    ctx.invokeid = ctx.createInvokeId
    return ctx
}
