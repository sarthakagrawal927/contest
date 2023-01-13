const a = [];
const b = a;
b.push(1);
console.log({ a }) // [a] -> because b points to a, a is a value.


enum TSEnum {
    Foo,
    Bar,
    Blast
}

type CoolType = {
    bar?: string;
}

function doSomething(foo: CoolType): boolean {
    if (foo.bar) {
        return true
    } else return false
}