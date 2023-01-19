import getConfig, { Operation } from "../config";

// test("this is a test", () => {
//     expect(true).toEqual(false)
// })

test("simple print all", () => {
    const config = getConfig({
        args: ["foo"]
    })
    expect(config.operation).toEqual(Operation.Print)
    expect(config.args).toEqual(["foo"])
})

test("add key", () => {
    const config = getConfig({
        args: ["add", "foo", "bar"]
    })
    expect(config.operation).toEqual(Operation.Add)
    expect(config.args).toEqual(["foo", "bar"])
})

test("rm key", () => {
    const config = getConfig({
        args: ["rm", "foo"]
    })
    expect(config.operation).toEqual(Operation.Remove)
    expect(config.args).toEqual(["foo"])
})