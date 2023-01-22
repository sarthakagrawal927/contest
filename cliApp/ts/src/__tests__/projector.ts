import { Operation } from "../config";
import Projector from "../projector";

function getData() {
    return {
        projector: {
            "/": {
                "foo": "bar1",
                "disco": "dancer",
            },
            "/foo": {
                "foo": "bar2"
            },
            "/foo/bar": {
                "foo": "bar3"
            }
        }
    }
}

function getProjector(pwd: string, data = getData()): Projector {
    return new Projector({
        args: [],
        operation: Operation.Print,
        pwd,
        config: "Hello ghost"
    }, data);
}

test("getValueAll", () => {
    const proj = getProjector("/foo/bar")
    expect(proj.getAllValue()).toEqual({
        "foo": "bar3",
        "disco": "dancer",
    })
})

test("getValue", () => {
    let proj = getProjector("/foo/bar")
    expect(proj.getValue("foo")).toEqual("bar3")
    proj = getProjector("/foo")
    expect(proj.getValue("foo")).toEqual("bar2")
    expect(proj.getValue("disco")).toEqual("dancer")
})

test("getValue", () => {
    let data = getData();
    let proj = getProjector("/foo/bar", data)
    proj.setValue("foo", "baz")

    expect(proj.getValue("foo")).toEqual("baz")
    proj.setValue("disco", "trespasser")
    expect(proj.getValue("disco")).toEqual("trespasser")

    proj = getProjector("/", data)
    expect(proj.getValue("disco")).toEqual("dancer")
})

test("removeValue", () => {
    let proj = getProjector("/foo/bar")
    proj.removeValue("disco");
    expect(proj.getValue("disco")).toEqual("dancer")

    proj.removeValue("foo");
    expect(proj.getValue("foo")).toEqual("bar2")
})