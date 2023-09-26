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
    instruction_status: Option<String>,
}

impl Default for CPUDebugView {
    fn default() -> Self {
        Self {
            instructions: String::new(),
            cpu: CPU::new(),
            instruction_status: None,
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
                            Some(execute_instruction(&mut self.cpu, &self.instructions));
                    }
                });
                // Instruction status
                if let Some(status) = &self.instruction_status {
                    let color = if status == "Done!" {
                        Color32::GREEN
                    } else {
                        Color32::RED
                    };

                    ui.colored_label(color, status);
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

fn execute_instruction(cpu: &mut CPU, instruction_string: &String) -> String {
    let args = instruction_string.split_whitespace().collect::<Vec<_>>();
    let inst = args.get(0);
    let rd = args.get(1);
    let rs1 = args.get(2);
    let imm = args.get(3);

    if inst.is_some() && rd.is_some() && rs1.is_some() && imm.is_some() {
        let rd = rd.unwrap().replace("x", "").remove(0).to_digit(10).unwrap() as u8;
        let rs1 = rs1
            .unwrap()
            .replace("x", "")
            .remove(0)
            .to_digit(10)
            .unwrap() as u8;
        let _ = match inst.unwrap() {
            &"addi" => cpu.execute(Instruction {
                inst: isa::RV32I::ADDI,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 0,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            &"slti" => cpu.execute(Instruction {
                inst: isa::RV32I::SLTI,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 2,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            &"sltiu" => cpu.execute(Instruction {
                inst: isa::RV32I::SLTIU,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 3,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            &"xori" => cpu.execute(Instruction {
                inst: isa::RV32I::XORI,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 4,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            &"ori" => cpu.execute(Instruction {
                inst: isa::RV32I::ORI,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 6,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            &"andi" => cpu.execute(Instruction {
                inst: isa::RV32I::ANDI,
                inst_type: isa::InstructionType::I(isa::I {
                    imm: parse_imm(imm.unwrap()),
                    rs1: rs1,
                    funct3: 7,
                    rd: rd,
                    opcode: 0x13,
                }),
                raw: 0,
            }),

            _ => {
                return "Invalid instruction".to_string();
            }
        };
    } else {
        return "Invalid instruction".to_string();
    }
    return "Done!".to_string();
}

fn parse_imm(imm: &str) -> u32 {
    imm.parse().unwrap_or_else(|_| {
        let imm = i32::from_str_radix(imm, 16).unwrap();
        imm as u32
    })
}
