export function fileAbsoluteToRelative(base: string, file: string): string {
    if (base.startsWith("file://")) {
        base = base.substring(7);
    }
    if (file.startsWith("file://")) {
        file = file.substring(7);
    }
    if (base.startsWith("/") && file.startsWith("/")) {
        return fileAbsoluteToRelativeUnix(base, file);
    } else if (base.startsWith("/") && !file.startsWith("/")) {
        return fileAbsoluteToRelativeNt(base, file);
    } else if (!base.startsWith("/") && file.startsWith("/")) {
        return fileAbsoluteToRelativeNt(base, file);
    } else if (!base.startsWith("/") && !file.startsWith("/")) {
        return fileAbsoluteToRelativeUnix(base, file);
    } else {
        return file;
    }
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