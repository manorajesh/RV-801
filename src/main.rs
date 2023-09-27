use cpu::{ Interface, CPU };
use eframe::egui;
use egui::*;
use isa::Instruction;

mod cpu;
mod isa;
mod tests;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RV32I CPU",
        options,
        Box::new(|_cc| Box::<CPUDebugView>::default())
    )
}

struct CPUDebugView {
    instructions: String,
    cpu: CPU,
    instruction_status: Result<String, String>,
    instruction_history: Vec<String>,
}

impl Default for CPUDebugView {
    fn default() -> Self {
        Self {
            instructions: String::new(),
            cpu: init_cpu_test(),
            instruction_status: Ok(String::new()),
            instruction_history: Vec::new(),
        }
    }
}

impl eframe::App for CPUDebugView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Debug UI for CPU emulator
            ui.heading("RV32I CPU Emulator");
            ui.add_space(5.0);

            // Text entry for instructions
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Instructions:");
                    ui.text_edit_multiline(&mut self.instructions).context_menu(|ui| {
                        if ui.button("Clear").clicked() {
                            self.instructions.clear();
                        }
                    });
                    ui.horizontal(|ui| {
                        if
                            ui
                                .button("Run")
                                .on_hover_text("Execute the instruction (append to memory)")
                                .clicked()
                        {
                            let instructions = self.instructions.lines().collect();
                            self.instruction_status = execute_instructions(
                                &mut self.cpu,
                                instructions
                            );
                            if self.instruction_status.is_ok() {
                                self.instruction_history.extend(
                                    self.instructions.lines().map(|s| s.to_string())
                                );
                            }
                        }
                        if
                            ui
                                .small_button("Reset")
                                .on_hover_text("Reset CPU (clear registers, pc, and memory)")
                                .clicked()
                        {
                            self.cpu = init_cpu_test();
                            self.instruction_status = Ok(String::new());
                            self.instruction_history.clear();
                        }
                    })
                });

                // Instruction status
                match self.instruction_status {
                    Ok(ref status) => {
                        ui.colored_label(Color32::GREEN, status);
                    }
                    Err(ref status) => {
                        ui.colored_label(Color32::RED, status);
                    }
                }
            });

            ui.add_space(5.0);
            ui.label("Binary:").on_hover_text("Binary representation of last instruction");
            if let Some(last_inst) = self.cpu.last_inst {
                ui.label(format!("{:032b}", last_inst.to_bin()));
            } else {
                ui.label("00000000000000000000000000000000");
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.strong("CPU State");
            ui.add_space(5.0);

            // CPU state
            ScrollArea::vertical()
                .auto_shrink([false, true])
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Registers:").on_hover_text(
                                "The registers of the CPU (what each instruction operates on)"
                            );
                            // PC
                            ui.horizontal(|ui| {
                                ui.label("pc").on_hover_text(
                                    "Program Counter is the address of the next instruction to be executed"
                                );
                                ui.add_space(7.5);
                                ui.add(egui::DragValue::new(&mut self.cpu.pc)).on_hover_text(
                                    "Program Counter is the address of the next instruction to be executed"
                                );
                            });
                            for (i, reg) in self.cpu.regs.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    let signed = *reg as i32;
                                    ui.label(format!("x{:02}", i));
                                    ui.add(egui::DragValue::new(reg)).on_hover_text(
                                        "Unsigned value of register"
                                    );
                                    ui.colored_label(
                                        Color32::DARK_GRAY,
                                        format!("({})", signed)
                                    ).on_hover_text("Signed value of register");
                                });
                            }
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.vertical(|ui| {
                            ui.label("Memory:").on_hover_text(
                                "First 128 bytes of memory in hex (what the CPU reads from and writes to)"
                            );

                            // Memory
                            egui::Grid
                                ::new("memory_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    for (i, byte) in self.cpu.memory
                                        .iter_mut()
                                        .take(128)
                                        .enumerate() {
                                        if i % 4 == 0 && i > 0 {
                                            ui.end_row();
                                        }
                                        ui.add(
                                            egui::DragValue::new(byte).hexadecimal(1, false, true)
                                        ).on_hover_text(format!("Memory address: 0x{:08X}", i));
                                    }
                                });
                        });

                        // History
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ScrollArea::vertical()
                            .auto_shrink([false, true])
                            .stick_to_bottom(false)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label("History:").on_hover_text(
                                        "Instructions that have been executed"
                                    );
                                    for (i, inst) in self.instruction_history.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{:02}", i));
                                            // arrow right
                                            ui.label("—");
                                            ui.label(inst).on_hover_text(
                                                format!(
                                                    "{:032b}",
                                                    parse_i_instructions(inst)
                                                        .unwrap_or(Instruction::nop())
                                                        .to_bin()
                                                )
                                            );
                                        });
                                    }
                                })
                            });
                    });
                });
        });
    }
}

fn execute_instructions(cpu: &mut CPU, insts: Vec<&str>) -> Result<String, String> {
    let mut status = String::new();
    for inst in insts {
        let mnemonic = inst.split_whitespace().next().ok_or("Invalid instruction")?;
        match mnemonic {
            "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" => {
                status = execute_i_instruction(cpu, &inst)?;
            }
            _ => {
                return Err("Invalid instruction".into());
            }
        }
    }
    Ok(status)
}

fn parse_i_instructions(instruction_string: &str) -> Result<Instruction, String> {
    let args: Vec<_> = instruction_string.split_whitespace().collect();

    let (inst, rd, rs1, imm) = match args.as_slice() {
        [i, rd, rs1, imm, ..] => (i, rd, rs1, imm),
        _ => {
            return Err("Insufficient arguments or invalid instruction".into());
        }
    };

    let reg_match: &[_] = &[',', 'x'];
    let rd = rd
        .trim_matches(reg_match)
        .parse()
        .map_err(|_| format!("Failed to parse rd: {rd}"))?;
    let rs1 = rs1
        .trim_matches(reg_match)
        .parse()
        .map_err(|_| format!("Failed to parse rs1: {rs1}"))?;

    if rd > 31 {
        return Err("Invalid rd".into());
    }

    if rs1 > 31 {
        return Err("Invalid rs1".into());
    }

    let instruction = match *inst {
        "addi" => Instruction::new(isa::RV32I::ADDI, parse_imm(imm)?, rs1, 0, rd),
        "slti" => Instruction::new(isa::RV32I::SLTI, parse_imm(imm)?, rs1, 2, rd),
        "sltiu" => Instruction::new(isa::RV32I::SLTIU, parse_imm(imm)?, rs1, 3, rd),
        "xori" => Instruction::new(isa::RV32I::XORI, parse_imm(imm)?, rs1, 4, rd),
        "ori" => Instruction::new(isa::RV32I::ORI, parse_imm(imm)?, rs1, 6, rd),
        "andi" => Instruction::new(isa::RV32I::ANDI, parse_imm(imm)?, rs1, 7, rd),
        "slli" => Instruction::new(isa::RV32I::SLLI, parse_imm(imm)?, rs1, 1, rd),
        "srli" => Instruction::new(isa::RV32I::SRLI, parse_imm(imm)?, rs1, 5, rd),
        "srai" => Instruction::new(isa::RV32I::SRAI, parse_imm(imm)?, rs1, 5, rd),
        _ => {
            return Err("Invalid instruction".into());
        }
    };

    Ok(instruction)
}

fn execute_i_instruction(cpu: &mut CPU, instruction_string: &str) -> Result<String, String> {
    let instruction = parse_i_instructions(instruction_string)?;

    cpu.from_inst(vec![instruction.to_bin()]);
    cpu.run()?;

    Ok(format!("Executed instruction: {}", instruction_string))
}

impl Instruction {
    fn new(inst: isa::RV32I, imm: u32, rs1: i32, funct3: i32, rd: i32) -> Self {
        Self {
            inst,
            inst_type: isa::InstructionType::I(isa::I {
                imm,
                rs1: rs1 as u8,
                funct3: funct3 as u8,
                rd: rd as u8,
                opcode: 0x13,
            }),
            raw: 0,
        }
    }
}

fn parse_imm(imm: &str) -> Result<u32, String> {
    if let Ok(value) = imm.parse::<u32>() {
        return Ok(value);
    }

    if let Ok(value) = imm.parse::<i32>() {
        return Ok(value as u32);
    }

    Err(format!("Invalid immediate value (only numbers): {}", imm))
}

fn init_cpu_test() -> CPU {
    let mut cpu = CPU::new();
    cpu.exit_on_nop = true;
    cpu
}
