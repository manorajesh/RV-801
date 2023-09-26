use cpu::CPU;
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
        Box::new(|_cc| Box::<CPUDebugView>::default()),
    )
}

struct CPUDebugView {
    instructions: String,
    cpu: CPU,
    instruction_status: Result<String, String>,
}

impl Default for CPUDebugView {
    fn default() -> Self {
        Self {
            instructions: String::new(),
            cpu: CPU::new(),
            instruction_status: Ok(String::new()),
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
                    ui.text_edit_singleline(&mut self.instructions);
                    if ui
                        .button("Run")
                        .on_hover_text("Execute the instruction")
                        .clicked()
                    {
                        self.instruction_status =
                            execute_instruction(&mut self.cpu, &self.instructions);
                    }
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

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.strong("CPU State");
            ui.add_space(5.0);

            // Registers
            ScrollArea::vertical()
                .auto_shrink([false, true])
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Registers:").on_hover_text(
                                "The registers of the CPU (what each instruction operates on)",
                            );
                            for (i, reg) in self.cpu.regs.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    let signed = *reg as i32;
                                    ui.label(format!("x{:02}", i));
                                    ui.add(egui::DragValue::new(reg))
                                        .on_hover_text("Unsigned value of register");
                                    ui.colored_label(Color32::DARK_GRAY, format!("({})", signed))
                                        .on_hover_text("Signed value of register");
                                });
                            }
                        })
                    });
                });
        });
    }
}

fn execute_instruction(cpu: &mut CPU, instruction_string: &str) -> Result<String, String> {
    let args: Vec<_> = instruction_string.split_whitespace().collect();

    let (inst, rd, rs1, imm) = match args.as_slice() {
        [i, rd, rs1, imm, ..] => (i, rd, rs1, imm),
        _ => return Err("Insufficient arguments or invalid instruction".into()),
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
        "addi" => Instruction::new(isa::RV32I::ADDI, parse_imm(imm), rs1, 0, rd),
        "slti" => Instruction::new(isa::RV32I::SLTI, parse_imm(imm), rs1, 2, rd),
        "sltiu" => Instruction::new(isa::RV32I::SLTIU, parse_imm(imm), rs1, 3, rd),
        "xori" => Instruction::new(isa::RV32I::XORI, parse_imm(imm), rs1, 4, rd),
        "ori" => Instruction::new(isa::RV32I::ORI, parse_imm(imm), rs1, 6, rd),
        "andi" => Instruction::new(isa::RV32I::ANDI, parse_imm(imm), rs1, 7, rd),
        _ => return Err("Invalid instruction".into()),
    };

    if let Err(e) = cpu.execute(instruction) {
        return Err(format!("Failed to execute instruction: {}", e));
    }

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

fn parse_imm(imm: &str) -> u32 {
    imm.parse().unwrap_or_else(|_| {
        let imm = i32::from_str_radix(imm, 10).unwrap();
        imm as u32
    })
}
