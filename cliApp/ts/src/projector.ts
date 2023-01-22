import { Config } from "./config"
import * as fs from "fs";
import * as path from "path";

export type Data = {
    projector: {
        // pwd
        [key: string]: {
            // key -> value
            [key: string]: string
        }
    }
}

const defaultData: Data = {
    projector: {}
}

export default class Projector {

    constructor(private config: Config, private data: Data) { }

    getAllValue(): { [key: string]: string } {
        let curr = this.config.pwd;
        let prev = "";
        const paths = [];
        do {
            prev = curr;
            paths.push(curr)
            curr = path.dirname(curr)
        } while (curr !== prev)
        let out: { [key: string]: string } = paths.reverse().reduce((acc, path) => {
            const value = this.data.projector[path];
            if (value) {
                Object.assign(acc, value);
            }
            return acc;
        }, {})
        return out;
    }

    getValue(key: string): string | undefined {
        let curr = this.config.pwd;
        let prev = "";
        let out: string = undefined;
        do {
            const value = this.data.projector[curr]?.[key];
            if (value) {
                out = value;
                break;
            }
            prev = curr;
            curr = path.dirname(curr)
        } while (curr !== prev)
        return out;
    }

    setValue(key: string, val: string) {
        let dir = this.data.projector[this.config.pwd];
        if (!dir) {
            this.data.projector[this.config.pwd] = {}
        }
        dir[key] = val
    }

    removeValue(key: string) {
        let dir = this.data.projector[this.config.pwd];
        if (dir) {
            delete dir[key];
        }
    }

    static fromConfig(config: Config): Projector {
        if (fs.existsSync(config.config)) {
            let data: Data = undefined;
            try {
                data = JSON.parse(fs.readFileSync(config.config).toString())
            } catch (e) {
                data = defaultData
            }
            return new Projector(config, data);
        }
        return new Projector(config, defaultData);
    }
}