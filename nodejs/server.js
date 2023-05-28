const express = require('express')
const fileupload = require('express-fileupload')
const fs = require('fs')
const app = express()
const uuid = require("uuid")
const mysql = require("mysql");
const bcrypt = require("bcrypt")
const util = require("util")


const con = mysql.createConnection({
    host: "localhost",
	user: "root",
	password: "mongoDB",
	database: "research",
});

con.connect(err => {
    if (err) {
        throw err;
    }
})

const query = util.promisify(con.query).bind(con);

app.use(fileupload())

app.use('/files', express.static('files'))

app.post("/upload", async (req, res) => {
    const files = req.files;
    Object.entries(files).forEach(async (file) => {
        await file[1].mv("./uploads/"+uuid.v4())
    })
    res.sendStatus(200);
})

app.get("/create-user/:num", async (req, res) => {
    const num = req.params.num;

    
    for(let i = 0; i < num; i++){
        
        const salt = await bcrypt.genSalt(10);
        const hash = bcrypt.hashSync("1234567890-1234567890", salt);
        const sql = "insert into js_user(id, name, email, password) values (NULL, ?, ?, ?);"
        query({
            sql,
            values: ["Chris Kay", "chrisuser199@gmail.com", hash]
        })
    }
    
    res.sendStatus(200)
})

app.get("/heavy/:iter", (req,res) => {
    const bound = 100_000*req.params.iter
    for(let i = 0; i<=bound; i++){
        const sum = 1+1;  
    }

    res.sendStatus(200)
})

app.get("/create-post/:uid/:num", async (req,res) => {
    const {uid, num} = req.params

    for(let i = 0; i < num; i++) {
        const sql = "insert into js_post(pid, uid, title, body, tags) values (NULL, ?, ?, ?, ?);"
        await query({
            sql,
            values: [uid, "Hello world", "hello this is a sample, please follow me...", ["programming", "web servers", "software development"].join(" ")]
        })

    }
    res.sendStatus(200);
})

app.use("/", express.static("./dist"))

app.listen(9002)