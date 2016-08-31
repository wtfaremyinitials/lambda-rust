var spawn = require('child_process').spawn

var outstanding = {}
var rust = spawn('./target/debug/rust-lambda')

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

    //context = {"callbackWaitsForEmptyEventLoop":true,"logGroupName":"/aws/lambda/lambdaRustTest","logStreamName":"2016/08/30/[$LATEST]f7ec8219892ab4bf584508fa03b11329","functionName":"lambdaRustTest","memoryLimitInMB":"128","functionVersion":"$LATEST","invokeid":"44bbd77b-20ad-4a71-a37a-a1d031d2e4bc","awsRequestId":"bb428486-a254-4e40-9672-e29a6fa40efb","invokedFunctionArn":"arn:aws:lambda:us-east-1:632094372989:function:lambdaRustTest"}

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
