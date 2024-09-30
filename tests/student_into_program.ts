import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {StudentIntoProgram} from "../target/types/student_into_program";
import {expect} from "chai";

describe("student_into_program", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.StudentIntoProgram as Program<StudentIntoProgram>;

    const student = {
        name: 'Jonathon Adler',
        message: "Hey I am John, Nice to meet you!"
    }

    const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(student.name), provider.wallet.publicKey.toBuffer()], program.programId)

    it("Student Intro is added", async () => {
        const tx = await program.methods.addStudentIntro(student.name, student.message).rpc();
        const account = await program.account.studentIntoState.fetch(studentPda);
        expect(account.name === student.name);
        expect(account.message === student.message);
        expect(account.student === provider.wallet.publicKey);
    });

    it("Student Intro is updated", async () => {
        const newMessage = "Hi there"
        const tx = await program.methods.updateStudentIntro(student.name, newMessage).rpc();
        const account = await program.account.studentIntoState.fetch(studentPda);
        expect(account.name === student.name);
        expect(account.message === newMessage);
        expect(account.student === provider.wallet.publicKey);
    });

    it("Student Intro is deleted", async () => {
        const tx = await program.methods.deleteStudentIntro(student.name).rpc();
    });
});
