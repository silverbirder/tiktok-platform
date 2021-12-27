(async () => {
    const baseDate = new Date("2019-12-27");
    const baseUrl = 'https://t.tiktok.com/api/user/list/';
    const urls = [];
    for (let i = 0; i < 365; i++) {
        const maxCursor = Math.floor(baseDate.getTime() / 1000);
        baseDate.setDate(baseDate.getDate() - 1);
        const minCursor = Math.floor(baseDate.getTime() / 1000);
        urls.push(`${baseUrl}?count=100&maxCursor=${maxCursor}&minCursor=${minCursor}`);
    }
    const responseList = await Promise.all(urls.map(async (url) => {
        return await (await fetch(url)).json();
    }));
    const userMap = {};
    responseList.filter((r) => {
        return r !== undefined && r.statusCode == 0;
    }).map((r) => {
        return r.userList;
    }).flat().filter((r) => {
        return r !== undefined;
    }).map((r) => {
        if (!userMap.hasOwnProperty(r.user.id)) {
            userMap[r.user.id] = {};
        }
        r['id'] = parseInt(r.user.id);
        userMap[r.user.id] = Object.assign(userMap[r.user.id], r);
    });
    const userKeys = Object.keys(userMap);
    const userList = userKeys.map((userKey) => {
        return userMap[userKey];
    });
    console.log(userList);
})()

// on https://t.tiktok.com/api/user/list/?count=15&maxCursor=1625875200&minCursor=1623283200