const fs = require('fs');
const currentDirJsonFiles = fs.readdirSync('.').filter((f) => /\.json$/.test(f));

const allUsers = currentDirJsonFiles.map((f) => {
    return JSON.parse(fs.readFileSync(f, 'utf8')).map((j) => j.user.uniqueId);
}).flat();
const uniqueAllUsers = [...new Set(allUsers)];

const reBuildAllUsers = uniqueAllUsers.map((u) => {
    return {
        "authorMeta": {
            "name": u
        }
    }
});

fs.writeFileSync('./output/output.json', JSON.stringify(reBuildAllUsers));