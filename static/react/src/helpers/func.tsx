import _ from 'lodash';
import * as React from 'react';
import ReactMarkdown from 'react-markdown/with-html';
import { Branch } from '../Types';
/*
    cycle.js
    2018-05-15

    Public Domain.

    NO WARRANTY EXPRESSED OR IMPLIED. USE AT YOUR OWN RISK.

    This code should be minified before deployment.
    See http://javascript.crockford.com/jsmin.html

    USE YOUR OWN COPY. IT IS EXTREMELY UNWISE TO LOAD CODE FROM SERVERS YOU DO
    NOT CONTROL.
*/

// The file uses the WeakMap feature of ES6.

if (typeof (JSON as any).decycle !== "function") {
    (JSON as any).decycle = function decycle(object: any, replacer: any) {
        "use strict";

        // Make a deep copy of an object or array, assuring that there is at most
        // one instance of each object or array in the resulting structure. The
        // duplicate references (which might be forming cycles) are replaced with
        // an object of the form

        //      {"$ref": PATH}

        // where the PATH is a JSONPath string that locates the first occurance.

        // So,

        //      var a = [];
        //      a[0] = a;
        //      return JSON.stringify(JSON.decycle(a));

        // produces the string '[{"$ref":"$"}]'.

        // If a replacer function is provided, then it will be called for each value.
        // A replacer function receives a value and returns a replacement value.

        // JSONPath is used to locate the unique object. $ indicates the top level of
        // the object or array. [NUMBER] or [STRING] indicates a child element or
        // property.

        const objects = new WeakMap();     // object to path mappings

        return (function derez(value, path) {

            // The derez function recurses through the object, producing the deep copy.

            let oldPath;   // The path of an earlier occurance of value
            let nu: any;         // The new object or array

            // If a replacer function was provided, then call it to get a replacement value.

            if (replacer !== undefined) {
                value = replacer(value);
            }

            // typeof null === "object", so go on if this value is really an object but not
            // one of the weird builtin objects.

            if (
                typeof value === "object"
                && value !== null
                && !(value instanceof Boolean)
                && !(value instanceof Date)
                && !(value instanceof Number)
                && !(value instanceof RegExp)
                && !(value instanceof String)
            ) {

                // If the value is an object or array, look to see if we have already
                // encountered it. If so, return a {"$ref":PATH} object. This uses an
                // ES6 WeakMap.

                oldPath = objects.get(value);
                if (oldPath !== undefined) {
                    return { $ref: oldPath };
                }

                // Otherwise, accumulate the unique value and its path.

                objects.set(value, path);

                // If it is an array, replicate the array.

                if (Array.isArray(value)) {
                    nu = [];
                    value.forEach((element, i) => {
                        nu[i] = derez(element, path + "[" + i + "]");
                    });
                } else {

                    // If it is an object, replicate the object.

                    nu = {};
                    Object.keys(value).forEach((name) => {
                        nu[name] = derez(
                            value[name],
                            path + "[" + JSON.stringify(name) + "]"
                        );
                    });
                }
                return nu;
            }
            return value;
        }(object, "$"));
    };
};

const match = (what: RegExp) => (obj: any, key: string) => {
    return what.test(obj[key]);
};

export interface Row {
    key: JSX.Element;
    keyString: string;
    value: JSX.Element;
    valueString: string;
    location: {
        pathname: string;
        search: string;
    }
    domain: Branch | undefined;
}

const routes = ['home', 'experiences', 'studies', 'skills', 'hobbies'];
const domains = [Branch.SOFTWARE, Branch.WOOD];

const search = (result: Row[]) => (obj: any, pattern: RegExp, prefix: string) => {
    return (a: string) => {
        if (typeof obj[a] === "object" && obj[a] !== null) {
            Object.keys(obj[a]).forEach(
                search(result)(
                    obj[a],
                    pattern,
                    (/\d+/.test(a)
                        ? `${prefix} #${a}`
                        : (prefix !== '' ? prefix + ' > ' : '')
                        + a)
                ));
        } else {
            if (match(pattern)(obj, a)) {
                const stringMatch = typeof obj[a] === 'string' ? obj[a].match(pattern) : null;
                const output = stringMatch ? stringMatch.input.replace(pattern, `<mark>${stringMatch[0]}</mark>`) : obj[a];
                const keyOutput = (prefix !== '' ? (prefix + ' > ') : '') + a;
                const foundRoute = routes.filter((localRoute: string) => {
                    return keyOutput.match(new RegExp(localRoute, 'i'));
                });
                const foundDomain = domains.filter((localDomain: Branch) => {
                    return keyOutput.match(new RegExp(localDomain));
                });
                const path = foundRoute.length > 0 ? `/${foundRoute[0]}` : null;
                const urlSearchParams = new URLSearchParams(window.location.search);
                urlSearchParams.set('hl', stringMatch ? stringMatch[0] : '');
                const domain = foundDomain ? foundDomain[0] : null;
                if (path && domain) {
                    const row: Row = {
                        key: <strong><div dangerouslySetInnerHTML={{ __html: keyOutput }} /></strong>,
                        keyString: keyOutput,
                        value: stringMatch && stringMatch.length > 0
                            ? <ReactMarkdown source={output} allowDangerousHtml={true} />
                            : <div>{obj[a]}</div>,
                        valueString: obj[a],
                        location: {
                            pathname: path === '/home' ? '/' : path,
                            search: urlSearchParams.toString()
                        },
                        domain,
                    }
                    result.push(row);
                }
            }
        }
    };
};

export const deepSearch = (where: any, what: RegExp) => {
    const o = (JSON as any).decycle(where);
    const result: Row[] = [];
    Object.keys(o).forEach(search(result)(o, what, ''));
    search(result)(where, what, '');
    return result;
};

interface HighlightedTextProps {
    text: string;
    highlight: string;
    className?: string;
}
export const HighlightedText = ({ text, highlight, className = '' }: HighlightedTextProps) => {
    if (!highlight.trim()) {
        return <span className={className}>{text}</span>
    }
    const regex = new RegExp(`(${_.escapeRegExp(highlight)})`, 'gi')
    const parts = text.split(regex)
    return (
        <span className={className}>
            {parts.filter(part => part).map((part, i) => (
                regex.test(part) ? <mark key={i}>{part}</mark> : <span key={i} className={className}>{part}</span>
            ))}
        </span>
    )
};

export const getHighlightKeyword = (query: string): string => {
    const params = new URLSearchParams(query);
    const hl = params.get('hl');
    return hl || '';
};

export const addMarkToMarkDown = (md: string, hl: string): string => {
    return hl ? md.replace(new RegExp(hl, 'i'), `<mark>${hl}</mark>`) : md;
}
