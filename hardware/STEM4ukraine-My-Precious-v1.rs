ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=LjOMK/PbzYK/2HBB46gAAAAN;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAAAO; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							device=led5
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAAC6; loclib_name=3mmLED_backplane;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane.lht
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=LjOMK/PbzYK/2HBB46gAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.57 {
				uuid=LjOMK/PbzYK/2HBB46gAAADH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=52000; y=48000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED2
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.58 {
				uuid=LjOMK/PbzYK/2HBB46gAAADK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=28000; y=48000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED1
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.59 {
				uuid=LjOMK/PbzYK/2HBB46gAAADN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=28000; y=88000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220
				}
			}
			ha:group.60 {
				uuid=LjOMK/PbzYK/2HBB46gAAADQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=52000; y=88000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220
				}
			}
			ha:group.61 {
				uuid=LjOMK/PbzYK/2HBB46gAAADT;
				x=0; y=8000;
				li:objects {
					ha:line.2 { x1=52000; y1=84000; x2=52000; y2=80000; stroke=wire; }
					ha:line.3 { x1=28000; y1=84000; x2=28000; y2=80000; stroke=wire; }
					ha:line.4 { x1=28000; y1=84000; x2=28000; y2=84000; stroke=junction; }
					ha:line.5 { x1=12000; y1=84000; x2=52000; y2=84000; stroke=wire; }
					ha:line.7 { x1=16000; y1=88000; x2=16000; y2=84000; stroke=wire; }
					ha:line.8 { x1=16000; y1=84000; x2=16000; y2=84000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.65 {
				uuid=LjOMK/PbzYK/2HBB46gAAADU;
				x=-36000; y=8000;
				li:objects {
					ha:line.1 { x1=64000; y1=56000; x2=64000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.68 {
				uuid=LjOMK/PbzYK/2HBB46gAAADV;
				x=-36000; y=8000;
				li:objects {
					ha:line.1 { x1=88000; y1=56000; x2=88000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.92 {
				uuid=ukggpUEc4cgsieO3pZUAAAAh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=16000; y=44000;
				li:objects {
					ha:group.1 {
						uuid=ukggpUEc4cgsieO3pZUAAAAi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.94 {
				uuid=bjFYgiRVXUAzLVGUgpIAAAAf;
				li:objects {
					ha:line.1 { x1=28000; y1=44000; x2=28000; y2=48000; stroke=wire; }
					ha:line.3 { x1=28000; y1=44000; x2=28000; y2=44000; stroke=junction; }
					ha:line.4 { x1=16000; y1=44000; x2=52000; y2=44000; stroke=wire; }
					ha:line.5 { x1=52000; y1=44000; x2=52000; y2=48000; stroke=wire; }
					ha:line.6 { x1=16000; y1=44000; x2=16000; y2=80000; stroke=wire; }
					ha:line.7 { x1=16000; y1=80000; x2=12000; y2=80000; stroke=wire; }
					ha:line.9 { x1=12000; y1=76000; x2=16000; y2=76000; stroke=wire; }
					ha:line.10 { x1=16000; y1=76000; x2=16000; y2=76000; stroke=junction; }
					ha:line.11 { x1=12000; y1=72000; x2=16000; y2=72000; stroke=wire; }
					ha:line.12 { x1=16000; y1=72000; x2=16000; y2=72000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.97 {
				li:conn {
					/2/92/1/1
					/2/94/4
					/2/94/6
				}
			}
			ha:group.99 {
				uuid=bjFYgiRVXUAzLVGUgpIAAABQ; src_uuid=bjFYgiRVXUAzLVGUgpIAAABJ;
				x=8000; y=92000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABR; src_uuid=bjFYgiRVXUAzLVGUgpIAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABS; src_uuid=bjFYgiRVXUAzLVGUgpIAAABL;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABT; src_uuid=bjFYgiRVXUAzLVGUgpIAAABM;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABU; src_uuid=bjFYgiRVXUAzLVGUgpIAAABN;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABV; src_uuid=bjFYgiRVXUAzLVGUgpIAAABO;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABW; src_uuid=bjFYgiRVXUAzLVGUgpIAAABP;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=USB_B_180_degree_PTH_universal-v1.rf
					name=USB
					role=symbol
					spice/omit=yes
				}
			}
			ha:connection.100 {
				li:conn {
					/2/94/7
					/2/99/5/1
				}
			}
			ha:connection.102 {
				li:conn {
					/2/94/9
					/2/99/6/1
				}
			}
			ha:connection.103 {
				li:conn {
					/2/94/11
					/2/99/7/1
				}
			}
			ha:connection.104 {
				li:conn {
					/2/61/5
					/2/99/2/1
				}
			}
			ha:connection.105 {
				li:conn {
					/2/61/2
					/2/60/2/1
				}
			}
			ha:connection.106 {
				li:conn {
					/2/61/3
					/2/59/2/1
				}
			}
			ha:connection.107 {
				li:conn {
					/2/65/1
					/2/58/2/1
				}
			}
			ha:connection.108 {
				li:conn {
					/2/65/1
					/2/59/1/1
				}
			}
			ha:connection.109 {
				li:conn {
					/2/68/1
					/2/57/2/1
				}
			}
			ha:connection.110 {
				li:conn {
					/2/68/1
					/2/60/1/1
				}
			}
			ha:connection.111 {
				li:conn {
					/2/94/1
					/2/58/1/1
				}
			}
			ha:connection.112 {
				li:conn {
					/2/94/5
					/2/57/1/1
				}
			}
			ha:group.113 {
				uuid=bjFYgiRVXUAzLVGUgpIAAABb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=16000; y=96000;
				li:objects {
					ha:group.1 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:connection.114 {
				li:conn {
					/2/61/7
					/2/113/1/1
				}
			}
			ha:group.115 {
				uuid=bjFYgiRVXUAzLVGUgpIAAABg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=80000; y=96000;
				li:objects {
					ha:group.1 {
						uuid=bjFYgiRVXUAzLVGUgpIAAABh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.116 {
				uuid=bjFYgiRVXUAzLVGUgpIAAABi; src_uuid=bjFYgiRVXUAzLVGUgpIAAABf;
				x=84000; y=88000;
				li:objects {
					ha:line.1 { x1=-4000; y1=8000; x2=-4000; y2=4000; stroke=wire; }
					ha:line.3 { x1=0; y1=4000; x2=-4000; y2=4000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.117 {
				li:conn {
					/2/116/1
					/2/115/1/1
				}
			}
			ha:group.118 {
				uuid=bjFYgiRVXUAzLVGUgpIAAACF; src_uuid=bjFYgiRVXUAzLVGUgpIAAACC;
				x=88000; y=92000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=bjFYgiRVXUAzLVGUgpIAAACG; src_uuid=bjFYgiRVXUAzLVGUgpIAAACD;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=bjFYgiRVXUAzLVGUgpIAAACH; src_uuid=bjFYgiRVXUAzLVGUgpIAAACE;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXT_5V
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.119 {
				uuid=bjFYgiRVXUAzLVGUgpIAAACK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=80000; y=84000;
				li:objects {
					ha:group.1 {
						uuid=bjFYgiRVXUAzLVGUgpIAAACL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.120 {
				uuid=bjFYgiRVXUAzLVGUgpIAAACM;
				li:objects {
					ha:line.1 { x1=84000; y1=88000; x2=80000; y2=88000; stroke=wire; }
					ha:line.2 { x1=80000; y1=88000; x2=80000; y2=84000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.121 {
				li:conn {
					/2/120/1
					/2/118/3/1
				}
			}
			ha:connection.122 {
				li:conn {
					/2/120/2
					/2/119/1/1
				}
			}
			ha:connection.123 {
				li:conn {
					/2/116/3
					/2/118/2/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=80000
			drawing_min_width=90000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 2
     grid = 4.0960 mm
    }
   }
  }
}
