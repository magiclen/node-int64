declare module "int64-napi" {
    export function random(data: Buffer | string, errorCollection: number): Buffer[] | false;
}