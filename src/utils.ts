export function fileAbsoluteToRelative(base: string, file: string): string {
    if (base.startsWith("file://")) {
        base = base.substring(7);
    }
    if (file.startsWith("file://")) {
        file = file.substring(7);
    }
    if (base.includes("\\")) {
        return fileAbsoluteToRelativeNt(base, file)
    }
    return fileAbsoluteToRelativeUnix(base, file)
}

function fileAbsoluteToRelativeUnix(base: string, file: string): string {
    const baseParts = base.split("/");
    const fileParts = file.split("/");
    let i = 0;
    while (i < baseParts.length && i < fileParts.length && baseParts[i] === fileParts[i]) {
        i++;
    }
    return fileParts.slice(i).join("/");
}

function fileAbsoluteToRelativeNt(base: string, file: string): string {
    const baseParts = base.split("\\");
    const fileParts = file.split("\\");
    let i = 0;
    while (i < baseParts.length && i < fileParts.length && baseParts[i] === fileParts[i]) {
        i++;
    }
    return fileParts.slice(i).join("/");
}
export function workspaceUrlToName(url: string): string {
    if (url.includes("\\")) {
        return workspaceUrlToNameNt(url)
    }
    return workspaceUrlToNameUnix(url)
}

function workspaceUrlToNameUnix(url: string): string {
    const parts = url.split("/");
    return parts[parts.length - 1];
}

function workspaceUrlToNameNt(url: string): string {
    const parts = url.split("\\");
    return parts[parts.length - 1];
}

export function fpPos(path: string, list: string[]): number {
    // replace all \\ with /
    path = path.replaceAll("\\", "/");
    let i = 0;
    for (const item of list) {
        if (item.replaceAll("\\", "/").includes(path)) {
            return i;
        }

        i++;
    }

    return -1;
}

export function makeSureEndsInSlash(path: string): string {
    if (!path.endsWith("/")) {
        return path + "/";
    }
    return path;
}