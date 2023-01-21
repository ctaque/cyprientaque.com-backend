declare module "*.elm" {
    interface IElmType {
        Elm: {
            Components: any
        }
    }
    export const Elm: IElmType;
}
