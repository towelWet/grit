use std::fs;use std::path::Path;use std::process::Command;
#[test] fn test_init_and_save(){
    let dir=tempfile::tempdir().unwrap();let root=dir.path();
    run_grit(root,&["init"]);assert!(root.join(".grit").exists());
    fs::write(root.join("hello.txt"),b"hello").unwrap();run_grit(root,&["save","-m","first"]);
    let out=run_grit(root,&["status"]);assert!(out.contains("(clean)")||out.is_empty());
    fs::write(root.join("hello.txt"),b"modified").unwrap();
    let out=run_grit(root,&["status"]);assert!(out.contains("hello.txt"));
    run_grit(root,&["save","-m","second"]);let out=run_grit(root,&["log"]);assert!(out.contains("second"));
}
#[test] fn test_restore(){
    let dir=tempfile::tempdir().unwrap();let root=dir.path();
    run_grit(root,&["init"]);fs::write(root.join("f1.txt"),b"version a").unwrap();
    let out=run_grit(root,&["save","-m","a"]);let id=out.split_whitespace().nth(2).unwrap_or("?").trim_end_matches(',');
    fs::write(root.join("f1.txt"),b"version b").unwrap();run_grit(root,&["save","-m","b"]);
    run_grit(root,&["restore",id,"--force"]);assert_eq!(fs::read_to_string(root.join("f1.txt")).unwrap(),"version a");
}
#[test] fn test_branch(){
    let dir=tempfile::tempdir().unwrap();let root=dir.path();
    run_grit(root,&["init"]);fs::write(root.join("m.txt"),b"main").unwrap();run_grit(root,&["save","-m","main"]);
    run_grit(root,&["branch","create","feat"]);let out=run_grit(root,&["branch"]);assert!(out.contains("feat"));
    run_grit(root,&["checkout","feat"]);fs::write(root.join("f.txt"),b"feat").unwrap();run_grit(root,&["save","-m","feat"]);
    run_grit(root,&["checkout","main"]);assert!(!root.join("f.txt").exists());
}
#[test] fn test_verify(){let dir=tempfile::tempdir().unwrap();let root=dir.path();
    run_grit(root,&["init"]);fs::write(root.join("d.bin"),b"data").unwrap();run_grit(root,&["save","-m","data"]);
    let out=run_grit(root,&["verify"]);assert!(out.contains("valid"));
}
#[test] fn test_doctor(){let dir=tempfile::tempdir().unwrap();let root=dir.path();
    run_grit(root,&["init"]);let out=run_grit(root,&["doctor"]);assert!(out.contains("Repo:"));
}
fn run_grit(root:&Path,args:&[&str])->String{
    let bin=std::env::current_exe().unwrap().parent().unwrap().join("grit");
    let o=Command::new(&bin).args(args).arg("--repo").arg(root).output().expect("grit");
    String::from_utf8_lossy(if o.status.success(){&o.stdout}else{&o.stderr}).to_string()
}
