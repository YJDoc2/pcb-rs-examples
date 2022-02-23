// instructions are :
// 0 : stop
// 1 : read next byte addr and store in reg1
// 2 : read next byte addr and store in reg2
// 3 : write reg1 to next addr
// 4 : write reg2 to next addr
// 5 : add reg1 and reg2 and store in reg1

function compile(code){
    let _code = code.toLowerCase();
    let codelines = _code.split("\n");
    compiled = [];

    for (line in codelines){
        if(codelines[line].trim() == ''){
            continue;
        }
        let three_ac = codelines[line].split(" ");
        if(three_ac.length > 3){
            throw "Invalid code";
        }
        let cmd = three_ac[0];
        switch(cmd){
            case "hlt":{
                compiled.push(0); 
                continue;
            }
            case "mov":{
                let op1 = three_ac[1];
                let op2 = three_ac[2];
                // this is register
                if (op1 == "a"){
                    let addr = parseInt(op2);
                    compiled.push(1);
                    compiled.push(addr);
                    continue;
                }
                // this is register
                if (op1 == "b"){
                    let addr = parseInt(op2);
                    compiled.push(2);
                    compiled.push(addr);
                    continue;
                }
                let addr = parseInt(op1);
                // this is an addr
                if(op2 =="a"){
                    compiled.push(3);
                    compiled.push(addr);
                    continue;
                }
                if(op2 =="b"){                    
                    compiled.push(4);
                    compiled.push(addr);
                    continue;
                }

            }
            case "add":{
                compiled.push(5);
                continue;
            }
            deafult:{
                throw "invalid code";
            }
        }
    }
    return compiled;

}

let code = `
mov a 50
mov b 51
add
mov 52 a
hlt
`
console.log(compile(code));
